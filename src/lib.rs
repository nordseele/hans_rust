/*mod eurorack;
use crate::eurorack::*;

#[macro_export]

macro_rules! ii {
    ($module:ident, $unit:expr, $port:expr, $cmd:ident, $arg:ident) => {
        {ii::send_i2c(EuroModules::$module, $unit, $port, Some(er301::$cmd), vec![$arg as u16]).ok();}
    };
}

fn blabla() {
    let pitch = 12;
    ii!(Er301, 1, 3, CV, pitch);
}
*/
//ii::send_i2c(EuroModules::Er301, 1, 1, Some(er301::TR), vec![1]).ok();
