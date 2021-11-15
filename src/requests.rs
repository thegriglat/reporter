use crate::Shift;

use chrono::NaiveDateTime;

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

pub fn print_lhc_fills(conn: &oracle::Connection, start: NaiveDateTime, end: NaiveDateTime) {
    /*
        itext += hn("LHC fills statistics", 2)
    # last 1 day
    ocur.execute(
      "select NVL(lhcfill, 0), id, begintime, endtime, name, integratedlumi, integratedlivelumi from CMS_RUNTIME_LOGGER.RUNTIME_SUMMARY where (begintime >= :startq or endtime >= :startq or endtime is NULL) and (endtime is NULL or begintime <= :endq or endtime <= :endq)",
      startq=startdate, endq=enddate)
    tableh = ("LHCFILL", "Start", "End", "Name", "Integrated Lumi", "Integrated Live Lumi", "Link")
    tabled = []
    for row in sorted(ocur.fetchall(), key=lambda x: x[0], reverse=True):
      tabled.append([row[0]] + list(row[2:-2]) + [row[-2] / 1000.0, row[-1] / 1000.0] + [
        link("link", "https://cmswbm.web.cern.ch/cmswbm/cmsdb/servlet/FillRuntimeChart?runtimeID=" + str(row[1]))])

    if len(tabled) != 0:
      itext += table(tableh, tabled)
    else:
      itext += p("No LHC fills found")

        */
    println!("## LHC fills statistics");
    let sql = "select NVL(lhcfill, 0), begintime, endtime, name, integratedlumi, integratedlivelumi from CMS_RUNTIME_LOGGER.RUNTIME_SUMMARY where (begintime >= TO_DATE(:1, 'DD-MM-YYYY HH24:MI:SS') or endtime >= TO_DATE(:2,'DD-MM-YYYY HH24:MI:SS') or endtime is NULL) and (endtime is NULL or begintime <= TO_DATE(:3, 'DD-MM-YYYY HH24:MI:SS') or endtime <= TO_DATE(:4, 'DD-MM-YYYY HH24:MI:SS'))";
    let rows = conn.query(
        sql,
        &[
            &start.format("%d-%m-%Y %H:%M:%S").to_string(),
            &start.format("%d-%m-%Y %H:%M:%S").to_string(),
            &end.format("%d-%m-%Y %H:%M:%S").to_string(),
            &end.format("%d-%m-%Y %H:%M:%S").to_string(),
        ],
    );
    match rows {
        Ok(rw) => {
            println!(
                "| LHCFILL |  Start | End | Name | Integrated Lumi | Integrated Live Lumi | Link |"
            );
            println!("|---|---|---|---|---|---|---|");
            for row in rw {
                match row {
                    Ok(r) => {
                        let fill: String = r.get("fill").unwrap_or("-".to_string());
                        let begintime: String = r.get("begintime").unwrap_or("-".to_string());
                        let endtime: String = r.get("endtime").unwrap_or("-".to_string());
                        let name: String = r.get("name").unwrap_or("-".to_string());
                        let integratedlumi: String =
                            r.get("integratedlumi").unwrap_or("-".to_string());
                        let integratedlivelumi: String =
                            r.get("integratedlivelumi").unwrap_or("-".to_string());
                        let mut link: String = "https://cmswbm.web.cern.ch/cmswbm/cmsdb/servlet/FillRuntimeChart?runtimeID=".to_owned();
                        link.push_str(&fill);
                        println!(
                            "| {} | {} | {} | {} | {} | {} | {} |",
                            fill,
                            begintime,
                            endtime,
                            name,
                            integratedlumi,
                            integratedlivelumi,
                            link
                        );
                    }
                    Err(e) => {
                        println!("err: {}", e);
                    }
                };
            }
        }
        Err(e) => {
            println!("No LHC fills found: {}", e);
        }
    }
}
