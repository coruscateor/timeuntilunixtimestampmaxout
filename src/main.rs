use time::OffsetDateTime;

fn main()
{

    let utc_now = OffsetDateTime::now_utc();

    let uts = utc_now.unix_timestamp();

    println!("current unix timestamp: {}\n", uts);

    let time_left = i64::MAX - uts;

    println!("time left until i64::MAX: {}\n", time_left);

    //let as_u64: u64 = uts.into();

    // ^^^^ the trait `From<i64>` is not implemented for `u64`

    let try_res = u64::try_from(uts);

    match try_res
    {

        Ok(res) => {

            let u64_uts: u64  = res;

            let u64_time_left = u64::MAX - u64_uts;

            println!("time left until u64::MAX: {}\n", u64_time_left);

        },
        Err(err) => {

            println!("{}", err.to_string());

        }

    }

}
