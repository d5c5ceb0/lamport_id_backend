use crate::{
    common::error::{AppResult,AppError},
    database::{
        entities::{prelude::Vote, vote},
        Storage,
    },
};
use sea_orm::*;

impl Storage {
    pub async fn create_vote(&self, active_vote: vote::ActiveModel) -> AppResult<vote::Model> {
        tracing::info!("vote model: {:?}", active_vote);

        let created_vote = active_vote.insert(self.conn.as_ref()).await?;

        Ok(created_vote)
    }

    //get votes by proposal_id
    pub async fn get_votes_by_proposal_id(&self, proposal_id: &str, offset: i64, limit: i64) -> AppResult<Vec<vote::Model>> {
        Ok(Vote::find()
            .filter(vote::Column::ProposalId.eq(proposal_id))
            .offset(offset as u64)
            .limit(limit as u64)
            .all(self.conn.as_ref())
            .await?)

    }

    //get votes by voter_id
    pub async fn get_votes_by_voter_id(&self, voter_id: &str, offset: i64, limit: i64) -> AppResult<Vec<vote::Model>> {
        Ok(Vote::find()
            .filter(vote::Column::VoterId.eq(voter_id))
            .offset(offset as u64)
            .limit(limit as u64)
            .all(self.conn.as_ref())
            .await?)
    }

    //count votes by proposal_id and choice
    pub async fn count_votes_by_proposal_id_and_choice(&self, proposal_id: &str, choice: &str) -> AppResult<u64> {
        let count = Vote::find()
            .filter(vote::Column::ProposalId.eq(proposal_id))
            .filter(vote::Column::Choice.eq(choice))
            .count(self.conn.as_ref())
            .await?;

        Ok(count)
    }

    //count votes by group_id, group_id has many proposals, proposals has many votes
    pub async fn count_votes_by_group_id(&self, group_id: &str) -> AppResult<i64> {
        let count = self.conn.query_one(Statement::from_string(
                        self.conn.get_database_backend(),
                        format!("SELECT COUNT(v.id) FROM vote v INNER JOIN proposals p ON v.proposal_id = p.proposal_id WHERE p.group_id = \'{}\';", group_id),
        )).await?.unwrap().try_get_by::<i64, _>(0).unwrap();


        Ok(count)
    }

    pub async fn count_voters_by_group_id(&self, group_id: &str) -> AppResult<i64> {
        let count = self.conn.query_one(Statement::from_string(
                        self.conn.get_database_backend(),
                        format!("SELECT COUNT(DISTINCT v.voter_id) FROM vote v INNER JOIN proposals p ON v.proposal_id = p.proposal_id WHERE p.group_id = \'{}\';", group_id),
        )).await?.unwrap().try_get_by::<i64, _>(0).unwrap();

        Ok(count)
    }

    pub async fn count_votes_by_proposal_id(&self, proposal_id: &str) -> AppResult<u64> {
        let count = Vote::find()
            .filter(vote::Column::ProposalId.eq(proposal_id))
            .count(self.conn.as_ref())
            .await?;

        Ok(count)
    }

    pub async fn is_voted_by_voter_id(&self, voter_id: &str, proposal_id: &str) -> AppResult<bool> {
        let count = Vote::find()
            .filter(vote::Column::VoterId.eq(voter_id))
            .filter(vote::Column::ProposalId.eq(proposal_id))
            .count(self.conn.as_ref())
            .await?;

        Ok(count > 0)
    }

    pub async fn get_proposal_vote_by_voter_id(&self, voter_id: &str, proposal_id: &str) -> AppResult<vote::Model> {
        match Vote::find()
            .filter(vote::Column::VoterId.eq(voter_id))
            .filter(vote::Column::ProposalId.eq(proposal_id))
            .one(self.conn.as_ref())
            .await? {
                Some(m) => Ok(m),
                None => Err(AppError::CustomError("vote is not existed".to_string()))
            }
    }

    //count vote by voter_id
    pub async fn count_votes_by_voter_id(&self, voter_id: &str) -> AppResult<u64> {
        let count = Vote::find()
            .filter(vote::Column::VoterId.eq(voter_id))
            .count(self.conn.as_ref())
            .await?;

        Ok(count)
    }

    //is voter in group 
    pub async fn is_voter_in_group(&self, voter_id: &str, group_id: &str) -> AppResult<bool> {
        let count = self.conn.query_one(Statement::from_string(
                        self.conn.get_database_backend(),
                        format!("SELECT COUNT(DISTINCT v.voter_id) FROM vote v INNER JOIN proposals p ON v.proposal_id = p.proposal_id WHERE p.group_id = \'{}\' AND v.voter_id = \'{}\';", group_id, voter_id),
        )).await?.unwrap().try_get_by::<i64, _>(0).unwrap();

        Ok(count > 0)
    }

}

