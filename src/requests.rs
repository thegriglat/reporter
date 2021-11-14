use crate::Shift;

pub fn print_shifter_info(shift: Shift, conn: &oracle::Connection) {
    let sql = "select shift_type_id from CMS_SHIFTLIST.SHIFT_TYPE where shift_type = :1 and sub_system = :2";
    let sql_shifter_ccid = "select shifter_id from CMS_SHIFTLIST.SHIFT_ROSTER where SHIFT_TYPE_ID = :1 and sysdate between shift_start and shift_end";
    const NO_SHIFTER_STR: &str = "no shifter for this period";
    let shifter_type_id: u32 = match conn.query_row(sql, &[&shift.shift, &shift.system]) {
        Ok(v) => match v.get("shift_type_id") {
            Ok(v) => v,
            Err(e) => {
                println!("Cannot get shift_type_id!");
                println!("{}", e);
                return;
            }
        },
        Err(e) => {
            println!("Cannot query CMS_SHIFTLIST.SHIFT_TYPE");
            println!("{}", e);
            return;
        }
    };
    // row.get("shift_type_id").unwrap();
    let shifter_ccid: String = match conn.query_row(sql_shifter_ccid, &[&shifter_type_id]) {
        Ok(v) => match v.get("shifter_id") {
            Ok(v1) => v1,
            Err(_e) => NO_SHIFTER_STR.to_string(),
        },
        Err(_e) => NO_SHIFTER_STR.to_string(),
    };

    println!(
        "Current *{}* {}: {}",
        shift.system, shift.shift, shifter_ccid
    );
}
