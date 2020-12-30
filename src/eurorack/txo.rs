pub use super::Command;
pub use super::Arg;
pub use super::Bufsize;
pub use super::txo;

// addresses

pub const ADDRESSES: [usize; 8] = [0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67];

// commands

pub enum Cmd {
    Tr, 
    TrTog,
    TrPulse,
    TrTime,
    TrPol,
    Cv,
    CvSlew,
    CvSet,
    CvOff,
    TrTimeS,
    TrTimeM,
    TrPulseDiv,
    TrM,
    TrMS,
    TrMM,
    TrMBpm,
    TrMAct,
    TrMSync,
    TrMWidth,
    TrMCount,
    TrMMul,
    TrMMute,
    CvSlewS,
    CvSlewM,
    CvQt,
    CvQtSet,
    CvN,
    CvNSet,
    CvScale,
    CvLog,
    Osc,
    OscSet,
    OscQt,
    OscQtSet,
    OscN,
    OscNSet,
    OscFq,
    OscFqSet,
    OscLfo,
    OscLfoSet,
    OscWave,
    OscSync,
    OscPhase,
    OscWidth,
    OscRect,
    OscSlew,
    OscSlewS,
    OscSlewM,
    OscScale,
    OscCyc,
    OscCycM,
    OscCycS,
    OscCycSet,
    OscCycMSet,
    OscCycSSet,
    OscCtr,
    Env,
    EnvAct,
    EnvAtt,
    EnvAttM,
    EnvAttS,
    EnvDec,
    EnvDecM,
    EnvDecS,
    EnvTrig,
    EnvEoc,
    EnvEor,
    EnvLoop,
    Kill,
    CvInit,
    CvCalib,
    CvReset,
    Init,
}

pub struct Module {
   pub tr       : Command,
   pub tr_tog   : Command,
   pub tr_pulse : Command,
   pub tr_time  : Command,
   pub tr_pol   : Command,
   pub cv       : Command,
   pub cv_slew  : Command,
   pub cv_set   : Command,
   pub cv_offset: Command,
   pub tr_time_s: Command,
   pub tr_time_m: Command,
   pub tr_pulse_div: Command,
   pub tr_m: Command,
   pub tr_m_s: Command,
   pub tr_m_m: Command,
   pub tr_m_bpm: Command,
   pub tr_m_act: Command,
   pub tr_m_sync: Command,
   pub tr_m_width: Command,
   pub tr_m_count: Command,
   pub tr_m_mul: Command,
   pub tr_m_mute: Command,
   pub cv_slew_s: Command,
   pub cv_slew_m: Command,
   pub cv_qt: Command,
   pub cv_qt_set: Command,
   pub cv_n: Command,
   pub cv_n_set: Command,
   pub cv_scale: Command,
   pub cv_log: Command,
   pub osc: Command,
   pub osc_set: Command,
   pub osc_qt: Command,
   pub osc_qt_set: Command,
   pub osc_n: Command,
   pub osc_n_set: Command,
   pub osc_fq: Command,
   pub osc_fq_set: Command,
   pub osc_lfo: Command,
   pub osc_lfo_set: Command,
   pub osc_wave: Command,
   pub osc_sync: Command,
   pub osc_phase: Command,
   pub osc_width: Command,
   pub osc_rect: Command,
   pub osc_slew: Command,
   pub osc_slew_s: Command,
   pub osc_slew_m: Command,
   pub osc_scale: Command,
   pub osc_cyc: Command,
   pub osc_cyc_m: Command,
   pub osc_cyc_s: Command,
   pub osc_cyc_set: Command,
   pub osc_cyc_m_set: Command,
   pub osc_cyc_s_set: Command,
   pub osc_ctr: Command,
   pub env: Command,
   pub env_act: Command,
   pub env_att: Command,
   pub env_att_m: Command,
   pub env_att_s: Command,
   pub env_dec: Command,
   pub env_dec_m: Command,
   pub env_dec_s: Command,
   pub env_trig: Command,
   pub env_eoc: Command,
   pub env_eor: Command,
   pub env_loop: Command,
   pub kill: Command,
   pub cv_init: Command,
   pub cv_calib: Command,
   pub cv_reset: Command,
   pub init: Command,
}

// basic commands
pub const TR: Command = Command {command_number: 0x0, args: &[ Arg{ name: "port", argtype: Bufsize::U8}, Arg{ name: "state", argtype: Bufsize::S16}], required:1};
pub const TR_TOG: Command = Command {command_number: 0x01, args: &[ Arg{ name: "port", argtype: Bufsize::U8}], required:0};
pub const TR_PULSE: Command = Command {command_number: 0x5, args: &[ Arg{ name: "port", argtype: Bufsize::U8}], required:0};
pub const TR_TIME: Command = Command {command_number: 0x32, args: &[ Arg{ name: "port", argtype: Bufsize::U8}, Arg{ name: "time", argtype: Bufsize::S16}], required:1};
pub const TR_POL: Command = Command {command_number: 0x6, args: &[ Arg{ name: "port", argtype: Bufsize::U8}, Arg{ name: "polarity", argtype: Bufsize::S16}], required:1};
pub const CV: Command = Command {command_number: 0x10, args: &[ Arg{ name: "port", argtype: Bufsize::U8}, Arg{ name: "volt", argtype: Bufsize::S16V}], required:1};
pub const CV_SLEW: Command = Command {command_number: 0x12, args: &[ Arg{ name: "port", argtype: Bufsize::U8}, Arg{ name: "time", argtype: Bufsize::S16V}], required:1};
pub const CV_SET: Command = Command {command_number: 0x11, args: &[ Arg{ name: "port", argtype: Bufsize::U8}, Arg{ name: "volt", argtype: Bufsize::S16V}], required:1};
pub const CV_OFFSET: Command = Command {command_number: 0x15, args: &[ Arg{ name: "port", argtype: Bufsize::U8}, Arg{ name: "offset", argtype: Bufsize::S16V}], required:1};
pub const TR_TIME_S: Command = Command {command_number: 0x3, args: &[ Arg{ name: "port", argtype: Bufsize::U8}, Arg{ name: "s", argtype: Bufsize::S16}], required:1};
pub const TR_TIME_M: Command = Command {command_number: 0x4, args: &[ Arg{ name: "port", argtype: Bufsize::U8}, Arg{ name: "m", argtype: Bufsize::S16}], required:1};
pub const TR_PULSE_DIV: Command = Command {command_number: 0x7, args: &[ Arg{ name: "port", argtype: Bufsize::U8}, Arg{ name: "pulses", argtype: Bufsize::S16}], required:1};
pub const TR_M: Command = Command {command_number: 0x8, args: &[ Arg{ name: "port", argtype: Bufsize::U8}, Arg{ name: "ms", argtype: Bufsize::S16}], required:1};
pub const TR_M_S: Command = Command {command_number: 0x9, args: &[ Arg{ name: "port", argtype: Bufsize::U8}, Arg{ name: "s", argtype: Bufsize::S16}], required:1};
pub const TR_M_M: Command = Command {command_number: 0x0A, args: &[ Arg{ name: "port", argtype: Bufsize::U8}, Arg{ name: "m", argtype: Bufsize::S16}], required:1};
pub const TR_M_BPM: Command = Command {command_number: 0x0B, args: &[ Arg{ name: "port", argtype: Bufsize::U8}, Arg{ name: "bpm", argtype: Bufsize::S16}], required:1};
pub const TR_M_ACT: Command = Command {command_number: 0x0C, args: &[ Arg{ name: "port", argtype: Bufsize::U8}, Arg{ name: "state", argtype: Bufsize::S16}], required:1};
pub const TR_M_SYNC: Command = Command {command_number: 0x0D, args: &[ Arg{ name: "port", argtype: Bufsize::U8}, Arg{ name: "state", argtype: Bufsize::S16}], required:1};
pub const TR_M_WIDTH: Command = Command {command_number: 0x0E, args: &[ Arg{ name: "port", argtype: Bufsize::U8}, Arg{ name: "width", argtype: Bufsize::S16}], required:1};
pub const TR_M_COUNT: Command = Command {command_number: 0x0F, args: &[ Arg{ name: "port", argtype: Bufsize::U8}, Arg{ name: "count", argtype: Bufsize::S16}], required:1};
pub const TR_M_MUL: Command = Command {command_number: 0x17, args: &[ Arg{ name: "port", argtype: Bufsize::U8}, Arg{ name: "mult", argtype: Bufsize::S16}], required:1};
pub const TR_M_MUTE: Command = Command {command_number: 0x16, args: &[ Arg{ name: "port", argtype: Bufsize::U8}, Arg{ name: "state", argtype: Bufsize::S16}], required:1};
pub const CV_SLEW_S: Command = Command {command_number: 0x13, args: &[ Arg{ name: "port", argtype: Bufsize::U8}, Arg{ name: "s", argtype: Bufsize::S16}], required:1};
pub const CV_SLEW_M: Command = Command {command_number: 0x14, args: &[ Arg{ name: "port", argtype: Bufsize::U8}, Arg{ name: "m", argtype: Bufsize::S16}], required:1};
pub const CV_QT: Command = Command {command_number: 0x30, args: &[ Arg{ name: "port", argtype: Bufsize::U8}, Arg{ name: "qt", argtype: Bufsize::S16}], required:1};
pub const CV_QT_SET: Command = Command {command_number: 0x31, args: &[ Arg{ name: "port", argtype: Bufsize::U8}, Arg{ name: "qt", argtype: Bufsize::S16}], required:1};
pub const CV_N: Command = Command {command_number: 0x32, args: &[ Arg{ name: "port", argtype: Bufsize::U8}, Arg{ name: "note", argtype: Bufsize::S16}], required:1};
pub const CV_N_SET: Command = Command {command_number: 0x33, args: &[ Arg{ name: "port", argtype: Bufsize::U8}, Arg{ name: "note", argtype: Bufsize::S16}], required:1};
pub const CV_SCALE: Command = Command {command_number: 0x34, args: &[ Arg{ name: "port", argtype: Bufsize::U8}, Arg{ name: "scale", argtype: Bufsize::S16}], required:1};
pub const CV_LOG: Command = Command {command_number: 0x35, args: &[ Arg{ name: "port", argtype: Bufsize::U8}, Arg{ name: "log", argtype: Bufsize::S16}], required:1};
pub const OSC: Command = Command {command_number: 0x40, args: &[ Arg{ name: "port", argtype: Bufsize::U8}, Arg{ name: "volts", argtype: Bufsize::S16V}], required:1};
pub const OSC_SET: Command = Command {command_number: 0x41, args: &[ Arg{ name: "port", argtype: Bufsize::U8}, Arg{ name: "volts", argtype: Bufsize::S16V}], required:1};
pub const OSC_QT: Command = Command {command_number: 0x42, args: &[ Arg{ name: "port", argtype: Bufsize::U8}, Arg{ name: "volts", argtype: Bufsize::S16V}], required:1};
pub const OSC_QT_SET: Command = Command {command_number: 0x43, args: &[ Arg{ name: "port", argtype: Bufsize::U8}, Arg{ name: "volts", argtype: Bufsize::S16V}], required:1};
pub const OSC_N: Command = Command {command_number: 0x46, args: &[ Arg{ name: "port", argtype: Bufsize::U8}, Arg{ name: "volts", argtype: Bufsize::S16V}], required:1};
pub const OSC_N_SET: Command = Command {command_number: 0x47, args: &[ Arg{ name: "port", argtype: Bufsize::U8}, Arg{ name: "volts", argtype: Bufsize::S16}], required:1};
pub const OSC_FQ: Command = Command {command_number: 0x44, args: &[ Arg{ name: "port", argtype: Bufsize::U8}, Arg{ name: "fq", argtype: Bufsize::S16}], required:1};
pub const OSC_FQ_SET: Command = Command {command_number: 0x45, args: &[ Arg{ name: "port", argtype: Bufsize::U8}, Arg{ name: "fq", argtype: Bufsize::S16}], required:1};
pub const OSC_LFO: Command = Command {command_number: 0x48, args: &[ Arg{ name: "port", argtype: Bufsize::U8}, Arg{ name: "fq", argtype: Bufsize::S16}], required:1};
pub const OSC_LFO_SET: Command = Command {command_number: 0x49, args: &[ Arg{ name: "port", argtype: Bufsize::U8}, Arg{ name: "fq", argtype: Bufsize::S16}], required:1};
pub const OSC_WAVE: Command = Command {command_number: 0x4A, args: &[ Arg{ name: "port", argtype: Bufsize::U8}, Arg{ name: "wave", argtype: Bufsize::S16}], required:1};
pub const OSC_SYNC: Command = Command {command_number: 0x4B, args: &[ Arg{ name: "port", argtype: Bufsize::U8}, Arg{ name: "wave", argtype: Bufsize::S16}], required:1};
pub const OSC_PHASE: Command = Command {command_number: 0x53, args: &[ Arg{ name: "port", argtype: Bufsize::U8}, Arg{ name: "phase", argtype: Bufsize::S16}], required:1};
pub const OSC_WIDTH: Command = Command {command_number: 0x4C, args: &[ Arg{ name: "port", argtype: Bufsize::U8}, Arg{ name: "width", argtype: Bufsize::S16}], required:1};
pub const OSC_RECT: Command = Command {command_number: 0x4D, args: &[ Arg{ name: "port", argtype: Bufsize::U8}, Arg{ name: "pol", argtype: Bufsize::S16}], required:1};
pub const OSC_SLEW: Command = Command {command_number: 0x4F, args: &[ Arg{ name: "port", argtype: Bufsize::U8}, Arg{ name: "ms", argtype: Bufsize::S16}], required:1};
pub const OSC_SLEW_S: Command = Command {command_number: 0x50, args: &[ Arg{ name: "port", argtype: Bufsize::U8}, Arg{ name: "s", argtype: Bufsize::S16}], required:1};
pub const OSC_SLEW_M: Command = Command {command_number: 0x51, args: &[ Arg{ name: "port", argtype: Bufsize::U8}, Arg{ name: "m", argtype: Bufsize::S16}], required:1};
pub const OSC_SCALE: Command = Command {command_number: 0x4E, args: &[ Arg{ name: "port", argtype: Bufsize::U8}, Arg{ name: "scale", argtype: Bufsize::S16}], required:1};
pub const OSC_CYC: Command = Command {command_number: 0x54, args: &[ Arg{ name: "port", argtype: Bufsize::U8}, Arg{ name: "ms", argtype: Bufsize::S16}], required:1};
pub const OSC_CYC_S: Command = Command {command_number: 0x55, args: &[ Arg{ name: "port", argtype: Bufsize::U8}, Arg{ name: "s", argtype: Bufsize::S16}], required:1};
pub const OSC_CYC_M: Command = Command {command_number: 0x56, args: &[ Arg{ name: "port", argtype: Bufsize::U8}, Arg{ name: "m", argtype: Bufsize::S16}], required:1};
pub const OSC_CYC_SET: Command = Command {command_number: 0x57, args: &[ Arg{ name: "port", argtype: Bufsize::U8}, Arg{ name: "ms", argtype: Bufsize::S16}], required:1};
pub const OSC_CYC_S_SET: Command = Command {command_number: 0x57, args: &[ Arg{ name: "port", argtype: Bufsize::U8}, Arg{ name: "s", argtype: Bufsize::S16}], required:1};
pub const OSC_CYC_M_SET: Command = Command {command_number: 0x59, args: &[ Arg{ name: "port", argtype: Bufsize::U8}, Arg{ name: "m", argtype: Bufsize::S16}], required:1};
pub const OSC_CTR: Command = Command {command_number: 0x5A, args: &[ Arg{ name: "port", argtype: Bufsize::U8}, Arg{ name: "ctr", argtype: Bufsize::S16}], required:1};
pub const ENV_ACT: Command = Command {command_number: 0x60, args: &[ Arg{ name: "port", argtype: Bufsize::U8}, Arg{ name: "state", argtype: Bufsize::S16}], required:1};
pub const ENV_ATT: Command = Command {command_number: 0x61, args: &[ Arg{ name: "port", argtype: Bufsize::U8}, Arg{ name: "ms", argtype: Bufsize::S16}], required:1};
pub const ENV_ATT_S: Command = Command {command_number: 0x62, args: &[ Arg{ name: "port", argtype: Bufsize::U8}, Arg{ name: "s", argtype: Bufsize::S16}], required:1};
pub const ENV_ATT_M: Command = Command {command_number: 0x63, args: &[ Arg{ name: "port", argtype: Bufsize::U8}, Arg{ name: "m", argtype: Bufsize::S16}], required:1};
pub const ENV_DEC: Command = Command {command_number: 0x64, args: &[ Arg{ name: "port", argtype: Bufsize::U8}, Arg{ name: "ms", argtype: Bufsize::S16}], required:1};
pub const ENV_DEC_S: Command = Command {command_number: 0x65, args: &[ Arg{ name: "port", argtype: Bufsize::U8}, Arg{ name: "s", argtype: Bufsize::S16}], required:1};
pub const ENV_DEC_M: Command = Command {command_number: 0x66, args: &[ Arg{ name: "port", argtype: Bufsize::U8}, Arg{ name: "m", argtype: Bufsize::S16}], required:1};
pub const ENV_TRIG: Command = Command {command_number: 0x67, args: &[ Arg{ name: "port", argtype: Bufsize::U8}, Arg{ name: "state", argtype: Bufsize::S16}], required:1};
pub const ENV_EOC: Command = Command {command_number: 0x6B, args: &[ Arg{ name: "port", argtype: Bufsize::U8}, Arg{ name: "state", argtype: Bufsize::S16}], required:1};
pub const ENV_EOR: Command = Command {command_number: 0x6A, args: &[ Arg{ name: "port", argtype: Bufsize::U8}, Arg{ name: "state", argtype: Bufsize::S16}], required:1};
pub const ENV_LOOP: Command = Command {command_number: 0x6C, args: &[ Arg{ name: "port", argtype: Bufsize::U8}, Arg{ name: "state", argtype: Bufsize::S16}], required:1};
pub const ENV: Command = Command {command_number: 0x6D, args: &[ Arg{ name: "port", argtype: Bufsize::U8}, Arg{ name: "state", argtype: Bufsize::S16}], required:1};
pub const KILL: Command = Command {command_number: 0x20, args: &[ Arg{ name: "port", argtype: Bufsize::U8}], required:0};
pub const TR_INIT: Command = Command {command_number: 0x22, args: &[ Arg{ name: "port", argtype: Bufsize::U8}], required:0};
pub const CV_INIT: Command = Command {command_number: 0x23, args: &[ Arg{ name: "port", argtype: Bufsize::U8}], required:0};
pub const CV_CALIB: Command = Command {command_number: 0x6E, args: &[ Arg{ name: "port", argtype: Bufsize::U8}], required:0};
pub const CV_RESET: Command = Command {command_number: 0x6F, args: &[ Arg{ name: "port", argtype: Bufsize::U8}], required:0};
pub const INIT: Command = Command {command_number: 0x24, args: &[ Arg{ name: "port", argtype: Bufsize::U8}], required:0};

// A TERMINER !! + Voir si les autres struct modules sont vraiment utilisés

pub fn cmd_from_string(cmd_name: &str) -> Option<Command>{
    match cmd_name {
     "tr"                 => Some(TR),
     "tr_tog"             => Some(TR_TOG),
     "tr_pulse" | "tr_p"  => Some(TR_PULSE),
     "tr_time"            => Some(TR_TIME),
     "tr_pol"             => Some(TR_POL),
     "cv"                 => Some(CV),
     "cv_slew"            => Some(CV_SLEW),
     "cv_set"             => Some(CV_SET),
     "cv_offset"          => Some(CV_OFFSET),
     "tr_time_s"          => Some(TR_TIME_S),
     "tr_time_m"          => Some(TR_TIME_M),
     "tr_pulse_div"       => Some(TR_PULSE_DIV),
     "tr_m"               => Some(TR_M),
     "tr_m_s"             => Some(TR_M_S),
     "tr_m_m"             => Some(TR_M_M),
     "tr_m_bpm"           => Some(TR_M_BPM),
     "tr_m_act"           => Some(TR_M_ACT),
     "tr_m_sync"          => Some(TR_M_SYNC),
     "tr_m_width"         => Some(TR_M_WIDTH),
     "tr_m_count"         => Some(TR_M_COUNT),
     "tr_m_mul"           => Some(TR_M_MUL),
     "tr_m_mute"          => Some(TR_M_MUTE),
     "cv_slew_s"          => Some(CV_SLEW_S),
     "cv_slew_m"          => Some(CV_SLEW_M),
     "cv_qt"              => Some(CV_QT),
     "cv_qt_set"          => Some(CV_QT_SET),
     "cv_n"               => Some(CV_N),
     "cv_n_set"           => Some(CV_N_SET),
     "cv_scale"           => Some(CV_SCALE),
     "cv_log"             => Some(CV_LOG),
     "osc"                => Some(OSC),
     "osc_set"            => Some(OSC_SET),
     "osc_qt"             => Some(OSC_QT),
     "osc_qt_set"         => Some(OSC_QT_SET),
     "osc_n"              => Some(OSC_N),
     "osc_n_set"          => Some(OSC_N_SET),
     "osc_fq"             => Some(OSC_FQ),
     "osc_fq_set"         => Some(OSC_FQ_SET),
     "osc_lfo"            => Some(OSC_LFO),
     "osc_lfo_set"        => Some(OSC_LFO_SET),
     "osc_wave"           => Some(OSC_WAVE),
     "osc_sync"           => Some(OSC_SYNC),
     "osc_phase"          => Some(OSC_PHASE),
     "osc_width"          => Some(OSC_WIDTH),
     "osc_rect"           => Some(OSC_RECT),
     "osc_slew"           => Some(OSC_SLEW),
     "osc_slew_s"         => Some(OSC_SLEW_S),
     "osc_slew_m"         => Some(OSC_SLEW_M),
     "osc_scale"          => Some(OSC_SCALE),
     "osc_cyc"            => Some(OSC_CYC),
     "osc_cyc_m"          => Some(OSC_CYC_M),
     "osc_cyc_s"          => Some(OSC_CYC_S),
     "osc_cyc_set"        => Some(OSC_CYC_SET),
     "osc_cyc_m_set"      => Some(OSC_CYC_M_SET),
     "osc_cyc_s_set"      => Some(OSC_CYC_S_SET),
     "osc_ctr"            => Some(OSC_CTR),
     "env"                => Some(ENV),
     "env_act"            => Some(ENV_ACT),
     "env_att"            => Some(ENV_ATT),
     "env_att_m"          => Some(ENV_ATT_M),
     "env_att_s"          => Some(ENV_ATT_S),
     "env_dec"            => Some(ENV_DEC),
     "env_dec_m"          => Some(ENV_DEC_M),
     "env_dec_s"          => Some(ENV_DEC_S),
     "env_trig"           => Some(ENV_TRIG),
     "env_eoc"            => Some(ENV_EOC),
     "env_eor"            => Some(ENV_EOR),
     "env_loop"           => Some(ENV_LOOP),
     "kill"               => Some(KILL),
     "cv_init"            => Some(CV_INIT),
     "cv_calib"           => Some(CV_CALIB),
     "cv_reset"           => Some(CV_RESET),
     "init"               => Some(INIT),
     _ => None,
    }
}


