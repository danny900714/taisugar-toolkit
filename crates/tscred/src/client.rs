use crate::error::Error;
use crate::{GetItemNeedsOptions, ItemNeeds, OperationCenter};
use ureq::Agent;

const GET_OPERATION_CENTERS_URL: &str =
    "http://192.168.41.30/TSCRED/BulkPeriodSheet/CennoDropdownList";
const GET_ITEM_NEEDS_URL: &str = "http://192.168.41.30/TSCRED/ItemNeedCount/GetItemNeedCount";

pub struct Client {
    agent: Agent,
}

impl Client {
    pub fn new(agent: Agent) -> Self {
        Self { agent }
    }

    pub fn get_operation_centers(&self) -> Result<Vec<OperationCenter>, Error> {
        Ok(self
            .agent
            .get(GET_OPERATION_CENTERS_URL)
            .call()?
            .body_mut()
            .read_json()?)
    }

    pub fn get_item_needs(&self, options: GetItemNeedsOptions<'_>) -> Result<ItemNeeds, Error> {
        Ok(self
            .agent
            .get(GET_ITEM_NEEDS_URL)
            .query("CLANA", "")
            .query("CLANA2", options.operation_center_id)
            .query("CLANO", options.start_date.strftime("%Y/%m/%d").to_string())
            .query("CLANO2", options.end_date.strftime("%Y/%m/%d").to_string())
            .query("DSP_SEL", options.display_mode.to_string())
            .query("HOST", options.department_id)
            .call()?
            .body_mut()
            .read_json()?)
    }
}
