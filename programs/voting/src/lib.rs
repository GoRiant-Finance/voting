use anchor_lang::prelude::*;

#[program]
pub mod voting_app {
    use super::*;
    use anchor_lang::prelude::borsh::maybestd::collections::HashMap;

    #[state]
    pub struct VoteState {
        pub owner: Pubkey,
        pub is_open: bool,
        pub proposals: HashMap<String, u8>,
        pub voters: HashMap<Pubkey, String>
    }

    impl VoteState {
        pub fn new(ctx: Context<Initialize>, proposals: Box<[String]>) -> Result<Self> {
            Ok(Self {
                owner: *ctx.accounts.owner.key,
                is_open: true,
                proposals: proposals.iter().fold(HashMap::new(), |mut acc, proposal| {
                    acc.insert(proposal.clone(), 0);
                    acc
                }),
                voters: HashMap::new()
            })
        }

        pub fn vote(&mut self, ctx: Context<Vote>, proposal_name: String) -> Result<()>{
            if self.is_open == false {
                return Err(ErrorCode::VotingClosed.into())
            }
            let voter = ctx.accounts.voter.key;
            let check_voted = self.voters.get(voter);
            match check_voted {
                Some(_) => Err(ErrorCode::Voted.into()),
                None => {
                    if let Some(pro) = self.proposals.get_mut(&*proposal_name) {
                        *pro += 1;
                        self.voters.insert(voter.clone(), proposal_name);
                        ()
                    }
                    Err(ErrorCode::InvalidVote.into())
                }
            }
        }

        pub fn close(&mut self, ctx: Context<CloseVote>) -> Result<()> {
            if *ctx.accounts.owner.key != self.owner {
                return Err(ErrorCode::PermissionDeny.into())
            }
            self.is_open = false;
            Ok(())
        }
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    pub owner: AccountInfo<'info>
}

#[derive(Accounts)]
pub struct Vote<'info> {
    #[account(mut)]
    pub voter: AccountInfo<'info>
}

#[derive(Accounts)]
pub struct CloseVote<'info> {
    #[account(mut)]
    pub owner: AccountInfo<'info>
}

#[error]
pub enum ErrorCode {
    #[msg("You can vote only once.")]
    Voted,
    #[msg("Your vote invalid")]
    InvalidVote,
    #[msg("Voting has been close.")]
    VotingClosed,
    #[msg("Permission deny")]
    PermissionDeny
}