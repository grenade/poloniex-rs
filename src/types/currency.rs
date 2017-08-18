use std::fmt;
use std::ascii::AsciiExt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Currency {
    #[serde(rename = "1CR")]
    Icr,
    Aby,
    Ac,
    Ach,
    Adn,
    Aeon,
    Aero,
    Air,
    Amp,
    Aph,
    Arch,
    Ardr,
    Aur,
    Axis,
    Balls,
    Bank,
    Bbl,
    Bbr,
    Bcc,
    Bch,
    Bcn,
    Bcy,
    Bdc,
    Bdg,
    Bela,
    Bitcny,
    Bits,
    Bitusd,
    Blk,
    Block,
    Blu,
    Bns,
    Bones,
    Bost,
    Btc,
    Btcd,
    Btcs,
    Btm,
    Bts,
    Burn,
    Burst,
    C2,
    Cach,
    Cai,
    Cc,
    Ccn,
    Cga,
    Cha,
    Cinni,
    Clam,
    Cnl,
    Cnmt,
    Cnote,
    Comm,
    Con,
    Corg,
    Crypt,
    Cure,
    Cyc,
    Dao,
    Dash,
    Dcr,
    Dgb,
    Dice,
    Diem,
    Dime,
    Dis,
    Dns,
    Doge,
    Drkc,
    Drm,
    Dsh,
    Dvk,
    Eac,
    Ebt,
    Ecc,
    Efl,
    Emc2,
    Emo,
    Enc,
    Etc,
    Eth,
    Exe,
    Exp,
    Fac,
    Fcn,
    Fct,
    Fibre,
    Flap,
    Fldc,
    Flo,
    Flt,
    Fox,
    Frac,
    Frk,
    Frq,
    Fvz,
    Fz,
    Fzn,
    Game,
    Gap,
    Gdn,
    Gemz,
    Geo,
    Giar,
    Glb,
    Gml,
    Gno,
    Gns,
    Gnt,
    Gold,
    Gpc,
    Gpuc,
    Grc,
    Grcx,
    Grs,
    Gue,
    H2o,
    Hiro,
    Hot,
    Huc,
    Huge,
    Hvc,
    Hyp,
    Hz,
    Ifc,
    Index,
    Ioc,
    Itc,
    Ixc,
    Jlh,
    Jpc,
    Jug,
    Kdc,
    Key,
    Lbc,
    Lc,
    Lcl,
    Leaf,
    Lgc,
    Lol,
    Love,
    Lqd,
    Lsk,
    Ltbc,
    Ltc,
    Ltcx,
    Maid,
    Mast,
    Max,
    Mcn,
    Mec,
    Meth,
    Mil,
    Min,
    Mint,
    Mmc,
    Mmnxt,
    Mmxiv,
    Mnta,
    Mon,
    Mrc,
    Mrs,
    Mts,
    Mun,
    Myr,
    Mzc,
    N5x,
    Nas,
    Naut,
    Nav,
    Nbt,
    Neos,
    Nl,
    Nmc,
    Nobl,
    Note,
    Noxt,
    Nrs,
    Nsr,
    Ntx,
    Nxc,
    Nxt,
    Nxti,
    Omni,
    Opal,
    Pand,
    Pasc,
    Pawn,
    Piggy,
    Pink,
    Plx,
    Pmc,
    Pot,
    Ppc,
    Prc,
    Prt,
    Pts,
    Q2c,
    Qbk,
    Qcn,
    Qora,
    Qtl,
    Rads,
    Rby,
    Rdd,
    Rep,
    Ric,
    Rzr,
    Sbd,
    Sc,
    Sdc,
    Shibe,
    Shopx,
    Silk,
    Sjcx,
    Slr,
    Smc,
    Soc,
    Spa,
    Sql,
    Srcc,
    Srg,
    Ssd,
    Steem,
    Str,
    Strat,
    Sum,
    Sun,
    Swarm,
    Sxc,
    Sync,
    Sys,
    Tac,
    Tor,
    Trust,
    Twe,
    Uis,
    Ultc,
    Unity,
    Uro,
    Usde,
    Usdt,
    Utc,
    Util,
    Uvc,
    Via,
    Voot,
    Vox,
    Vrc,
    Vtc,
    Wc,
    Wdc,
    Wiki,
    Wolf,
    X13,
    Xai,
    Xap,
    Xbc,
    Xc,
    Xch,
    Xcn,
    Xcp,
    Xcr,
    Xdn,
    Xdp,
    Xem,
    Xhc,
    Xlb,
    Xmg,
    Xmr,
    Xpb,
    Xpm,
    Xrp,
    Xsi,
    Xst,
    Xsv,
    Xusd,
    Xvc,
    Xxc,
    Yacc,
    Yang,
    Yc,
    Yin,
    Zec,
    #[serde(rename = "eTOK")]
    Etok
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = format!("{:?}", self).to_ascii_uppercase();
        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let btc = format!("{}", Currency::Btc);
        assert_eq!(btc, "BTC");

        let eth  = format!("{}", Currency::Eth);
        assert_eq!(eth, "ETH");
    }
}
