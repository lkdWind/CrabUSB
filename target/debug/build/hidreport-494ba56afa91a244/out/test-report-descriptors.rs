use hidreport::*;


    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_00DE_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:00DE.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:00DE.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_0531_0105_0003_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:0531:0105.0003.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:0531:0105.0003.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_521C_0003_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:521C.0003.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:521C.0003.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue769_0003_0B8C_0070_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue769-0003:0B8C:0070-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue769-0003:0B8C:0070-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_04F3_074D_0005_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:04F3:074D.0005.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:04F3:074D.0005.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue416_0018_0911_5288_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue416-0018:0911:5288-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue416-0018:0911:5288-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_51B9_0004_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:51B9.0004.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:51B9.0004.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue417_0018_056A_48ED_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue417-0018:056A:48ED-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue417-0018:056A:48ED-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue942_0003_046D_C03D_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue942-0003:046D:C03D-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue942-0003:046D:C03D-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_256C_006D_0020_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:256C:006D.0020.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:256C:006D.0020.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue664_0003_17EF_6085_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue664-0003:17EF:6085-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue664-0003:17EF:6085-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_51B7_0004_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:51B7.0004.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:51B7.0004.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_51BB_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:51BB.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:51BB.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_28BD_092B_000A_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:28BD:092B.000A.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:28BD:092B.000A.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_48C9_0008_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:48C9.0008.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:48C9.0008.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_28BD_0936_0007_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:28BD:0936.0007.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:28BD:0936.0007.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_517D_0004_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:517D.0004.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:517D.0004.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue921_0018_04F3_307A_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue921-0018:04F3:307A-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue921-0018:04F3:307A-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_51A0_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:51A0.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:51A0.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue785_0003_256C_006D_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue785-0003:256C:006D-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue785-0003:256C:006D-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0019_045E_09AF_0006_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0019:045E:09AF.0006.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0019:045E:09AF.0006.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_5196_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:5196.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:5196.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue712_0018_06CB_CD4F_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue712-0018:06CB:CD4F-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue712-0018:06CB:CD4F-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue613_0003_0951_16BE_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue613-0003:0951:16BE-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue613-0003:0951:16BE-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_49A3_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:49A3.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:49A3.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_28BD_092B_0009_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:28BD:092B.0009.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:28BD:092B.0009.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_03CE_008E_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:03CE.008E.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:03CE.008E.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_485B_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:485B.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:485B.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_045E_0745_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:045E:0745.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:045E:0745.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_488F_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:488F.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:488F.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_496C_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:496C.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:496C.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_50B6_0009_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:50B6.0009.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:50B6.0009.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_486A_0003_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:486A.0003.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:486A.0003.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056a_0114_0003_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056a:0114.0003.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056a:0114.0003.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_256C_5101_0004_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:256C:5101.0004.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:256C:5101.0004.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_31BE_0005_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:31BE.0005.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:31BE.0005.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue703_0005_20D6_6271_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue703-0005:20D6:6271-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue703-0005:20D6:6271-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_4814_0007_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:4814.0007.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:4814.0007.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue598_0018_06CB_CDEB_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue598-0018:06CB:CDEB-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue598-0018:06CB:CDEB-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_03CE_0005_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:03CE.0005.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:03CE.0005.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_5072_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:5072.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:5072.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_28BD_0905_0032_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:28BD:0905.0032.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:28BD:0905.0032.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_2BD6_0003_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:2BD6.0003.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:2BD6.0003.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue575_0003_048D_C955_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue575-0003:048D:C955-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue575-0003:048D:C955-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_27C6_0E51_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:27C6:0E51.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:27C6:0E51.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_045E_074F_0003_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:045E:074F.0003.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:045E:074F.0003.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue691_0018_06CB_7F8F_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue691-0018:06CB:7F8F-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue691-0018:06CB:7F8F-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_4806_0008_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:4806.0008.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:4806.0008.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_28BD_094B_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:28BD:094B.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:28BD:094B.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_5360_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:5360.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:5360.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056a_5044_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056a:5044.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056a:5044.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue581_0018_04F3_261A_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue581-0018:04F3:261A-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue581-0018:04F3:261A-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_2D1F_524C_003A_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:2D1F:524C.003A.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:2D1F:524C.003A.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_28BD_0913_0003_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:28BD:0913.0003.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:28BD:0913.0003.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_045E_07B2_0004_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:045E:07B2.0004.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:045E:07B2.0004.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue737_0018_0911_5288_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue737-0018:0911:5288-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue737-0018:0911:5288-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue352_0003_28BD_0077_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue352-0003:28BD:0077-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue352-0003:28BD:0077-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_28BD_0936_0009_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:28BD:0936.0009.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:28BD:0936.0009.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_51BF_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:51BF.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:51BF.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue356_0003_28BD_090D_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue356-0003:28BD:090D-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue356-0003:28BD:090D-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_51BD_0005_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:51BD.0005.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:51BD.0005.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_53A7_0003_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:53A7.0003.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:53A7.0003.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_509C_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:509C.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:509C.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_256C_006D_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:256C:006D.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:256C:006D.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_513B_0004_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:513B.0004.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:513B.0004.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_22CA_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:22CA.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:22CA.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue707_0018_06CB_CE44_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue707-0018:06CB:CE44-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue707-0018:06CB:CE44-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_51B8_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:51B8.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:51B8.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_5040_0005_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:5040.0005.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:5040.0005.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_4809_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:4809.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:4809.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_2BEB_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:2BEB.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:2BEB.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_264C_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:264C.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:264C.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue949_0003_046D_4085_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue949-0003:046D:4085-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue949-0003:046D:4085-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue666_0003_1AF3_0001_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue666-0003:1AF3:0001-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue666-0003:1AF3:0001-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_256C_006D_0006_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:256C:006D.0006.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:256C:006D.0006.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue717_0018_06CB_CE58_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue717-0018:06CB:CE58-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue717-0018:06CB:CE58-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_03ED_000A_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:03ED.000A.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:03ED.000A.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue743_0018_0EEF_C002_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue743-0018:0EEF:C002-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue743-0018:0EEF:C002-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_51B9_0003_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:51B9.0003.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:51B9.0003.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_0326_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:0326.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:0326.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue689_0018_0911_5288_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue689-0018:0911:5288-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue689-0018:0911:5288-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_4834_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:4834.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:4834.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_045E_09B5_0004_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:045E:09B5.0004.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:045E:09B5.0004.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_0114_0004_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:0114.0004.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:0114.0004.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_256C_006E_000F_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:256C:006E.000F.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:256C:006E.000F.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue482_0003_145F_0212_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue482-0003:145F:0212-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue482-0003:145F:0212-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue393_0018_04F3_24DB_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue393-0018:04F3:24DB-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue393-0018:04F3:24DB-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_0EEF_C002_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:0EEF:C002.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:0EEF:C002.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_51F6_0003_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:51F6.0003.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:51F6.0003.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_27C6_0111_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:27C6:0111.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:27C6:0111.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_51B7_0003_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:51B7.0003.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:51B7.0003.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_28BD_0913_0007_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:28BD:0913.0007.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:28BD:0913.0007.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue321_0003_045E_07E8_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue321-0003:045E:07E8-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue321-0003:045E:07E8-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_256C_006D_0021_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:256C:006D.0021.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:256C:006D.0021.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_03EC_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:03EC.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:03EC.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_256C_006D_000C_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:256C:006D.000C.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:256C:006D.000C.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_03ED_0008_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:03ED.0008.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:03ED.0008.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_03CB_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:03CB.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:03CB.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_27C6_01E0_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:27C6:01E0.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:27C6:01E0.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue970_0018_0911_5288_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue970-0018:0911:5288-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue970-0018:0911:5288-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_256C_0066_0018_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:256C:0066.0018.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:256C:0066.0018.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0019_045E_09B0_0005_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0019:045E:09B0.0005.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0019:045E:09B0.0005.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_4822_0008_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:4822.0008.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:4822.0008.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_0084_0003_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:0084.0003.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:0084.0003.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_3052_000A_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:3052.000A.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:3052.000A.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue695_0003_5543_0081_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue695-0003:5543:0081-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue695-0003:5543:0081-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue605_0018_04F3_303F_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue605-0018:04F3:303F-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue605-0018:04F3:303F-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_037A_000E_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:037A.000E.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:037A.000E.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_52C2_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:52C2.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:52C2.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_0319_0007_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:0319.0007.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:0319.0007.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_23F3_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:23F3.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:23F3.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_045E_0745_0003_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:045E:0745.0003.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:045E:0745.0003.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_51E2_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:51E2.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:51E2.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_50A0_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:50A0.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:50A0.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_5044_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:5044.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:5044.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue767_0003_046D_C08F_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue767-0003:046D:C08F-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue767-0003:046D:C08F-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_516B_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:516B.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:516B.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_51F5_0003_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:51F5.0003.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:51F5.0003.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue852_0003_046D_C091_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue852-0003:046D:C091-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue852-0003:046D:C091-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue499_0018_06CB_76AD_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue499-0018:06CB:76AD-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue499-0018:06CB:76AD-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_521A_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:521A.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:521A.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_51E3_0004_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:51E3.0004.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:51E3.0004.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_5157_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:5157.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:5157.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_00B1_0003_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:00B1.0003.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:00B1.0003.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_50FE_000B_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:50FE.000B.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:50FE.000B.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue486_0018_27C6_0118_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue486-0018:27C6:0118-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue486-0018:27C6:0118-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0005_056A_0379_0005_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0005:056A:0379.0005.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0005:056A:0379.0005.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue901_0005_056A_0377_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue901-0005:056A:0377-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue901-0005:056A:0377-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_50F1_0004_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:50F1.0004.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:50F1.0004.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_264C_0003_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:264C.0003.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:264C.0003.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_256C_006B_0003_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:256C:006B.0003.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:256C:006B.0003.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_5010_0003_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:5010.0003.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:5010.0003.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_51B0_0003_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:51B0.0003.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:51B0.0003.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue651_0005_17EF_60E1_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue651-0005:17EF:60E1-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue651-0005:17EF:60E1-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0019_045E_09AE_0004_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0019:045E:09AE.0004.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0019:045E:09AE.0004.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue887_0003_28BD_091F_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue887-0003:28BD:091F-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue887-0003:28BD:091F-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue303_0018_06CB_7A13_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue303-0018:06CB:7A13-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue303-0018:06CB:7A13-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_312C_0004_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:312C.0004.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:312C.0004.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_51B3_0004_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:51B3.0004.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:51B3.0004.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue519_0018_0488_121F_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue519-0018:0488:121F-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue519-0018:0488:121F-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_03A6_0009_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:03A6.0009.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:03A6.0009.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_045E_07A9_000F_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:045E:07A9.000F.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:045E:07A9.000F.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_03C0_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:03C0.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:03C0.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_31E6_0004_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:31E6.0004.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:31E6.0004.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_31AD_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:31AD.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:31AD.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_28BD_0933_0019_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:28BD:0933.0019.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:28BD:0933.0019.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_425B_0007_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:425B.0007.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:425B.0007.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_0084_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:0084.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:0084.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_304B_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:304B.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:304B.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_5543_004D_000D_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:5543:004D.000D.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:5543:004D.000D.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_056a_5002_0_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/056a:5002_0.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 056a:5002_0.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_2593_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:2593.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:2593.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue745_0003_17EF_608C_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue745-0003:17EF:608C-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue745-0003:17EF:608C-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_2FC2_000A_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:2FC2.000A.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:2FC2.000A.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_509D_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:509D.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:509D.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_5158_0005_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:5158.0005.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:5158.0005.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_2B0E_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:2B0E.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:2B0E.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_016C_0007_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:016C.0007.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:016C.0007.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_2CF9_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:2CF9.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:2CF9.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_5215_0003_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:5215.0003.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:5215.0003.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_03C0_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:03C0.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:03C0.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0005_045E_0827_0005_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0005:045E:0827.0005.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0005:045E:0827.0005.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_3139_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:3139.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:3139.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_4988_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:4988.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:4988.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_27C6_0113_0004_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:27C6:0113.0004.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:27C6:0113.0004.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_045E_0905_0005_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:045E:0905.0005.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:045E:0905.0005.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_52D5_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:52D5.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:52D5.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_5014_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:5014.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:5014.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_056a_010e_1_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/056a:010e_1.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 056a:010e_1.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_504A_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:504A.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:504A.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_5094_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:5094.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:5094.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_0124_0005_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:0124.0005.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:0124.0005.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_27C6_0113_0003_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:27C6:0113.0003.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:27C6:0113.0003.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_4911_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:4911.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:4911.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_303E_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:303E.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:303E.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_8191_0008_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:8191.0008.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:8191.0008.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_509D_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:509D.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:509D.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_0317_000A_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:0317.000A.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:0317.000A.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_2D1F_002C_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:2D1F:002C.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:2D1F:002C.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_3145_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:3145.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:3145.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_5256_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:5256.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:5256.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_528E_0003_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:528E.0003.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:528E.0003.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_495F_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:495F.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:495F.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_056a_5090_1_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/056a:5090_1.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 056a:5090_1.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_509F_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:509F.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:509F.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_2D1F_524C_0039_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:2D1F:524C.0039.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:2D1F:524C.0039.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_03EC_0003_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:03EC.0003.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:03EC.0003.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_03C4_0012_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:03C4.0012.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:03C4.0012.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue774_0018_2D1F_0095_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue774-0018:2D1F:0095-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue774-0018:2D1F:0095-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue556_0018_04F3_2AF4_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue556-0018:04F3:2AF4-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue556-0018:04F3:2AF4-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_3113_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:3113.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:3113.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_256C_006D_000B_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:256C:006D.000B.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:256C:006D.000B.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_504C_0007_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:504C.0007.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:504C.0007.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue986_0003_046D_4074_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue986-0003:046D:4074-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue986-0003:046D:4074-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_28BD_0071_0005_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:28BD:0071.0005.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:28BD:0071.0005.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_2DE2_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:2DE2.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:2DE2.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_03C0_0003_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:03C0.0003.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:03C0.0003.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue293_0018_044E_121F_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue293-0018:044E:121F-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue293-0018:044E:121F-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_27C6_011A_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:27C6:011A.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:27C6:011A.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue651_0003_17EF_60EE_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue651-0003:17EF:60EE-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue651-0003:17EF:60EE-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue340_0018_04F3_306A_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue340-0018:04F3:306A-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue340-0018:04F3:306A-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_8191_0009_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:8191.0009.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:8191.0009.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_51B8_0003_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:51B8.0003.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:51B8.0003.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_48CE_0007_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:48CE.0007.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:48CE.0007.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_033C_000C_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:033C.000C.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:033C.000C.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_5204_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:5204.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:5204.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue685_0018_04F3_30A6_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue685-0018:04F3:30A6-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue685-0018:04F3:30A6-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue919_0003_056A_00D3_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue919-0003:056A:00D3-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue919-0003:056A:00D3-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_48CA_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:48CA.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:48CA.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue844_0003_04B4_0711_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue844-0003:04B4:0711-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue844-0003:04B4:0711-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_056a_5002_1_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/056a:5002_1.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 056a:5002_1.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0044_045E_0021_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0044:045E:0021.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0044:045E:0021.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_484C_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:484C.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:484C.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue955_0003_484D_1002_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue955-0003:484D:1002-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue955-0003:484D:1002-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_2D1F_002E_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:2D1F:002E.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:2D1F:002E.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_48E2_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:48E2.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:48E2.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_50B7_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:50B7.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:50B7.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_0317_0009_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:0317.0009.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:0317.0009.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_51B6_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:51B6.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:51B6.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_0221_0025_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:0221.0025.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:0221.0025.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_04F3_0C79_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:04F3:0C79.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:04F3:0C79.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue318_0003_044E_120D_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue318-0003:044E:120D-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue318-0003:044E:120D-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_03CB_000E_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:03CB.000E.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:03CB.000E.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_3101_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:3101.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:3101.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_0318_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:0318.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:0318.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_525D_0003_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:525D.0003.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:525D.0003.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue889_0003_046D_4090_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue889-0003:046D:4090-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue889-0003:046D:4090-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_50FD_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:50FD.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:50FD.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_2BB1_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:2BB1.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:2BB1.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_5144_0018_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:5144.0018.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:5144.0018.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_256C_006D_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:256C:006D.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:256C:006D.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_045E_0904_0003_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:045E:0904.0003.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:045E:0904.0003.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue463_0018_04F3_30FD_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue463-0018:04F3:30FD-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue463-0018:04F3:30FD-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_23B9_0004_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:23B9.0004.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:23B9.0004.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_2BB3_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:2BB3.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:2BB3.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_2D1F_0095_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:2D1F:0095.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:2D1F:0095.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_056a_00e3_0_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/056a:00e3_0.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 056a:00e3_0.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_3293_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:3293.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:3293.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_0354_000A_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:0354.000A.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:0354.000A.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_03CE_0006_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:03CE.0006.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:03CE.0006.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_0318_0003_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:0318.0003.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:0318.0003.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_31F9_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:31F9.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:31F9.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_50B7_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:50B7.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:50B7.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue735_0003_0566_3027_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue735-0003:0566:3027-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue735-0003:0566:3027-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_27C6_0123_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:27C6:0123.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:27C6:0123.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_03F2_000F_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:03F2.000F.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:03F2.000F.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue925_0018_06CB_7813_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue925-0018:06CB:7813-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue925-0018:06CB:7813-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_51C7_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:51C7.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:51C7.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_256C_0064_001C_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:256C:0064.001C.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:256C:0064.001C.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_2D1F_0114_0003_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:2D1F:0114.0003.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:2D1F:0114.0003.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_04F3_2256_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:04F3:2256.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:04F3:2256.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_503F_0005_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:503F.0005.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:503F.0005.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_03F0_000C_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:03F0.000C.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:03F0.000C.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_0531_0102_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:0531:0102.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:0531:0102.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue201_0003_06A3_0C2D_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue201-0003:06A3:0C2D-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue201-0003:06A3:0C2D-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_3140_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:3140.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:3140.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_28BD_0071_0006_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:28BD:0071.0006.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:28BD:0071.0006.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_51B6_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:51B6.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:51B6.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue989_0003_256C_006E_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue989-0003:256C:006E-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue989-0003:256C:006E-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue723_0018_04F3_31BE_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue723-0018:04F3:31BE-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue723-0018:04F3:31BE-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue645_0003_09DA_9090_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue645-0003:09DA:9090-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue645-0003:09DA:9090-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_0374_000A_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:0374.000A.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:0374.000A.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_4875_0009_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:4875.0009.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:4875.0009.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_28BD_0936_0008_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:28BD:0936.0008.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:28BD:0936.0008.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue638_0003_046D_C092_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue638-0003:046D:C092-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue638-0003:046D:C092-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue721_0003_1C4F_0063_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue721-0003:1C4F:0063-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue721-0003:1C4F:0063-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue399_0005_004C_0265_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue399-0005:004C:0265-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue399-0005:004C:0265-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue644_0018_093A_0274_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue644-0018:093A:0274-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue644-0018:093A:0274-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_29B6_0003_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:29B6.0003.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:29B6.0003.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue566_0018_06CB_7F27_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue566-0018:06CB:7F27-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue566-0018:06CB:7F27-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue935_0018_04F3_22F7_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue935-0018:04F3:22F7-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue935-0018:04F3:22F7-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_304B_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:304B.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:304B.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_52B5_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:52B5.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:52B5.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_03D0_0003_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:03D0.0003.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:03D0.0003.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_5144_0017_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:5144.0017.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:5144.0017.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue354_0005_05AC_030E_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue354-0005:05AC:030E-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue354-0005:05AC:030E-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_0531_0104_0007_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:0531:0104.0007.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:0531:0104.0007.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue535_0018_06CB_CDE6_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue535-0018:06CB:CDE6-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue535-0018:06CB:CDE6-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_3138_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:3138.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:3138.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue964_0003_17EF_6047_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue964-0003:17EF:6047-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue964-0003:17EF:6047-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue804_0003_0853_0148_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue804-0003:0853:0148-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue804-0003:0853:0148-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_256C_006D_0003_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:256C:006D.0003.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:256C:006D.0003.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_5169_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:5169.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:5169.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_256C_5101_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:256C:5101.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:256C:5101.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_50B8_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:50B8.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:50B8.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue874_0005_004C_0265_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue874-0005:004C:0265-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue874-0005:004C:0265-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_045E_07A9_000E_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:045E:07A9.000E.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:045E:07A9.000E.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_4841_0006_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:4841.0006.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:4841.0006.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_4851_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:4851.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:4851.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue447_0005_05AC_030E_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue447-0005:05AC:030E-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue447-0005:05AC:030E-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0b05_179f_0_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0b05:179f_0.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0b05:179f_0.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue976_0018_0488_1063_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue976-0018:0488:1063-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue976-0018:0488:1063-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_5543_004D_000F_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:5543:004D.000F.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:5543:004D.000F.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_03C4_0010_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:03C4.0010.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:03C4.0010.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue771_0018_093A_0255_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue771-0018:093A:0255-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue771-0018:093A:0255-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_48EB_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:48EB.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:48EB.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_256C_0064_0003_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:256C:0064.0003.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:256C:0064.0003.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_056a_00e6_1_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/056a:00e6_1.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 056a:00e6_1.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_0326_0004_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:0326.0004.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:0326.0004.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_521F_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:521F.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:521F.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_24FE_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:24FE.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:24FE.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_50A0_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:50A0.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:50A0.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_5197_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:5197.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:5197.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_4898_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:4898.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:4898.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_481A_0004_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:481A.0004.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:481A.0004.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_04F3_2072_0004_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:04F3:2072.0004.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:04F3:2072.0004.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_5147_0005_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:5147.0005.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:5147.0005.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_5148_0007_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:5148.0007.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:5148.0007.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue387_0018_06CB_CD8B_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue387-0018:06CB:CD8B-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue387-0018:06CB:CD8B-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue400_0003_17EF_6047_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue400-0003:17EF:6047-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue400-0003:17EF:6047-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_5159_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:5159.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:5159.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_503E_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:503E.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:503E.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_511A_0004_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:511A.0004.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:511A.0004.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_03CB_0003_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:03CB.0003.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:03CB.0003.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_04F3_0757_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:04F3:0757.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:04F3:0757.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_0325_0003_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:0325.0003.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:0325.0003.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_0104_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:0104.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:0104.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_2D1F_0066_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:2D1F:0066.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:2D1F:0066.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_28BD_0928_0009_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:28BD:0928.0009.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:28BD:0928.0009.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_28BD_0935_000A_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:28BD:0935.000A.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:28BD:0935.000A.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_50B4_0006_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:50B4.0006.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:50B4.0006.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue410_0018_06CB_5F41_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue410-0018:06CB:5F41-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue410-0018:06CB:5F41-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue906_0003_1395_009A_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue906-0003:1395:009A-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue906-0003:1395:009A-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue848_0018_06CB_CE78_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue848-0018:06CB:CE78-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue848-0018:06CB:CE78-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_5019_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:5019.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:5019.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue403_0003_256C_006D_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue403-0003:256C:006D-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue403-0003:256C:006D-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_0531_0100_0013_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:0531:0100.0013.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:0531:0100.0013.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_27C6_0111_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:27C6:0111.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:27C6:0111.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_045E_00DB_0004_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:045E:00DB.0004.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:045E:00DB.0004.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue666_0003_046D_4054_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue666-0003:046D:4054-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue666-0003:046D:4054-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_52CC_0003_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:52CC.0003.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:52CC.0003.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_03CB_0004_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:03CB.0004.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:03CB.0004.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_5158_0004_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:5158.0004.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:5158.0004.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_525C_0005_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:525C.0005.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:525C.0005.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_0EEF_C066_0005_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:0EEF:C066.0005.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:0EEF:C066.0005.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_27C6_0113_0011_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:27C6:0113.0011.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:27C6:0113.0011.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue479_0003_046D_404D_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue479-0003:046D:404D-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue479-0003:046D:404D-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_29D4_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:29D4.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:29D4.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_03EB_8ABB_0006_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:03EB:8ABB.0006.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:03EB:8ABB.0006.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_28BD_0913_0005_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:28BD:0913.0005.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:28BD:0913.0005.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_04F3_2560_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:04F3:2560.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:04F3:2560.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_5543_004D_000E_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:5543:004D.000E.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:5543:004D.000E.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue576_0018_04F3_3098_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue576-0018:04F3:3098-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue576-0018:04F3:3098-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue781_0003_04B3_3109_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue781-0003:04B3:3109-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue781-0003:04B3:3109-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_03D0_0004_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:03D0.0004.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:03D0.0004.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_5150_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:5150.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:5150.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_0531_0100_0005_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:0531:0100.0005.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:0531:0100.0005.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_51B2_0007_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:51B2.0007.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:51B2.0007.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue829_0018_04F3_31C0_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue829-0018:04F3:31C0-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue829-0018:04F3:31C0-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_51BC_0006_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:51BC.0006.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:51BC.0006.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_0015_000E_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:0015.000E.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:0015.000E.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_50DB_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:50DB.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:50DB.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_00EC_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:00EC.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:00EC.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_4831_0006_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:4831.0006.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:4831.0006.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_504A_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:504A.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:504A.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue726_0005_004C_0265_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue726-0005:004C:0265-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue726-0005:004C:0265-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue647_0003_046D_404C_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue647-0003:046D:404C-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue647-0003:046D:404C-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_484D_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:484D.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:484D.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue342_0018_06CB_8323_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue342-0018:06CB:8323-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue342-0018:06CB:8323-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_00EC_0003_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:00EC.0003.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:00EC.0003.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue901_0005_056A_0377_1_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue901-0005:056A:0377-1.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue901-0005:056A:0377-1.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_03CF_0007_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:03CF.0007.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:03CF.0007.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue948_0018_056A_52C2_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue948-0018:056A:52C2-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue948-0018:056A:52C2-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_48CD_0005_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:48CD.0005.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:48CD.0005.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_056a_0117_0_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/056a:0117_0.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 056a:0117_0.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_2D1F_014B_0003_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:2D1F:014B.0003.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:2D1F:014B.0003.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_28BD_0928_0007_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:28BD:0928.0007.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:28BD:0928.0007.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_29CF_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:29CF.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:29CF.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue683_0003_1BCF_0005_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue683-0003:1BCF:0005-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue683-0003:1BCF:0005-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_256C_006D_000A_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:256C:006D.000A.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:256C:006D.000A.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue815_0003_12D1_10B8_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue815-0003:12D1:10B8-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue815-0003:12D1:10B8-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue514_0003_046D_C30F_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue514-0003:046D:C30F-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue514-0003:046D:C30F-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue855_0018_2D1F_0078_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue855-0018:2D1F:0078-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue855-0018:2D1F:0078-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_045E_07A9_0010_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:045E:07A9.0010.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:045E:07A9.0010.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_51B0_0004_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:51B0.0004.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:51B0.0004.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue580_0018_06CB_7621_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue580-0018:06CB:7621-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue580-0018:06CB:7621-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_4824_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:4824.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:4824.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_8191_000A_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:8191.000A.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:8191.000A.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue666_0003_046D_404A_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue666-0003:046D:404A-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue666-0003:046D:404A-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_00E6_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:00E6.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:00E6.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue306_0018_056A_4831_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue306-0018:056A:4831-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue306-0018:056A:4831-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_51B1_0003_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:51B1.0003.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:51B1.0003.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_52A2_0003_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:52A2.0003.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:52A2.0003.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_256C_006D_0010_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:256C:006D.0010.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:256C:006D.0010.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue802_0018_06CB_CD50_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue802-0018:06CB:CD50-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue802-0018:06CB:CD50-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue839_0003_28BD_0905_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue839-0003:28BD:0905-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue839-0003:28BD:0905-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_51EF_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:51EF.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:51EF.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_03C0_0004_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:03C0.0004.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:03C0.0004.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_51B1_0004_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:51B1.0004.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:51B1.0004.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue980_0005_05AC_030E_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue980-0005:05AC:030E-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue980-0005:05AC:030E-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue280_0005_004C_0265_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue280-0005:004C:0265-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue280-0005:004C:0265-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue776_0018_04F3_2DA1_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue776-0018:04F3:2DA1-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue776-0018:04F3:2DA1-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_312C_000A_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:312C.000A.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:312C.000A.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_503F_0004_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:503F.0004.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:503F.0004.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue316_0003_192F_0716_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue316-0003:192F:0716-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue316-0003:192F:0716-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue784_0018_093A_0255_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue784-0018:093A:0255-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue784-0018:093A:0255-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_51BB_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:51BB.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:51BB.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_51D0_0003_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:51D0.0003.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:51D0.0003.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue842_0003_28BD_091B_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue842-0003:28BD:091B-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue842-0003:28BD:091B-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue901_0005_056A_0379_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue901-0005:056A:0379-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue901-0005:056A:0379-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue618_0018_04F3_3185_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue618-0018:04F3:3185-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue618-0018:04F3:3185-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_2DA1_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:2DA1.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:2DA1.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_2A70_0008_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:2A70.0008.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:2A70.0008.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue749_0018_0488_120A_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue749-0018:0488:120A-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue749-0018:0488:120A-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_503E_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:503E.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:503E.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_318B_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:318B.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:318B.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_056a_0093_1_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/056a:0093_1.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 056a:0093_1.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_28BD_0933_0018_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:28BD:0933.0018.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:28BD:0933.0018.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_30F1_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:30F1.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:30F1.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_5148_0006_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:5148.0006.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:5148.0006.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_2D1F_0078_0004_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:2D1F:0078.0004.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:2D1F:0078.0004.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue470_0003_046D_1028_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue470-0003:046D:1028-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue470-0003:046D:1028-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_3052_0003_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:3052.0003.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:3052.0003.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue853_0018_2D1F_0095_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue853-0018:2D1F:0095-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue853-0018:2D1F:0095-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_5286_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:5286.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:5286.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_261A_0004_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:261A.0004.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:261A.0004.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_528A_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:528A.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:528A.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_04F3_0757_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:04F3:0757.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:04F3:0757.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_2CF7_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:2CF7.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:2CF7.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue996_0003_046D_C548_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue996-0003:046D:C548-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue996-0003:046D:C548-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_2A40_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:2A40.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:2A40.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_523A_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:523A.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:523A.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_50B8_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:50B8.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:50B8.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue699_0003_5543_0061_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue699-0003:5543:0061-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue699-0003:5543:0061-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_0027_000A_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:0027.000A.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:0027.000A.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_00F5_0_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:00F5_0.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:00F5_0.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_256C_0064_001D_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:256C:0064.001D.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:256C:0064.001D.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_0318_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:0318.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:0318.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_03EC_0005_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:03EC.0005.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:03EC.0005.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_2D1F_524C_0003_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:2D1F:524C.0003.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:2D1F:524C.0003.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue359_0018_0911_5288_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue359-0018:0911:5288-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue359-0018:0911:5288-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_5040_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:5040.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:5040.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue455_0003_1D57_AD03_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue455-0003:1D57:AD03-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue455-0003:1D57:AD03-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue522_0003_256C_006D_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue522-0003:256C:006D-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue522-0003:256C:006D-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_5229_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:5229.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:5229.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_49A5_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:49A5.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:49A5.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_256C_006E_0009_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:256C:006E.0009.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:256C:006E.0009.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_04F3_2674_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:04F3:2674.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:04F3:2674.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_28BD_0936_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:28BD:0936.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:28BD:0936.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0005_045E_0921_0007_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0005:045E:0921.0007.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0005:045E:0921.0007.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_256C_006E_000A_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:256C:006E.000A.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:256C:006E.000A.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_033C_000A_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:033C.000A.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:033C.000A.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0019_045E_0922_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0019:045E:0922.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0019:045E:0922.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_28BD_091F_001B_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:28BD:091F.001B.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:28BD:091F.001B.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_03F0_000A_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:03F0.000A.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:03F0.000A.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_52C6_0003_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:52C6.0003.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:52C6.0003.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue477_0003_28DE_1142_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue477-0003:28DE:1142-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue477-0003:28DE:1142-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue675_0003_046D_C01E_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue675-0003:046D:C01E-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue675-0003:046D:C01E-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_5284_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:5284.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:5284.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_50B4_0007_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:50B4.0007.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:50B4.0007.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_5285_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:5285.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:5285.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue626_0003_046D_C52E_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue626-0003:046D:C52E-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue626-0003:046D:C52E-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_3052_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:3052.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:3052.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_045E_07BD_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:045E:07BD.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:045E:07BD.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue349_0003_045E_009D_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue349-0003:045E:009D-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue349-0003:045E:009D-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_48EC_0006_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:48EC.0006.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:48EC.0006.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_2D1F_0040_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:2D1F:0040.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:2D1F:0040.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_2D1F_0185_0003_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:2D1F:0185.0003.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:2D1F:0185.0003.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_31E6_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:31E6.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:31E6.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_4865_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:4865.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:4865.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue513_0018_056A_5113_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue513-0018:056A:5113-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue513-0018:056A:5113-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_03C4_0013_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:03C4.0013.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:03C4.0013.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_22F7_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:22F7.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:22F7.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue331_0003_056A_5158_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue331-0003:056A:5158-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue331-0003:056A:5158-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_5276_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:5276.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:5276.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue493_0018_04F3_29F5_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue493-0018:04F3:29F5-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue493-0018:04F3:29F5-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue623_0018_04F3_3052_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue623-0018:04F3:3052-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue623-0018:04F3:3052-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_2813_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:2813.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:2813.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_320F_0004_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:320F.0004.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:320F.0004.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_28BD_092B_000B_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:28BD:092B.000B.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:28BD:092B.000B.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_5150_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:5150.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:5150.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_03CF_008F_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:03CF.008F.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:03CF.008F.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_2D1F_011E_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:2D1F:011E.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:2D1F:011E.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue632_0018_056A_5196_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue632-0018:056A:5196-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue632-0018:056A:5196-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_00E3_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:00E3.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:00E3.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_49A5_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:49A5.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:49A5.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_03CE_0090_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:03CE.0090.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:03CE.0090.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue891_0003_056A_501E_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue891-0003:056A:501E-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue891-0003:056A:501E-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_2D1F_0136_0003_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:2D1F:0136.0003.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:2D1F:0136.0003.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_0027_000B_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:0027.000B.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:0027.000B.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_04F3_074D_0006_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:04F3:074D.0006.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:04F3:074D.0006.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_304B_0005_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:304B.0005.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:304B.0005.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_51BC_0005_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:51BC.0005.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:51BC.0005.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_49C3_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:49C3.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:49C3.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue808_0018_04F3_319B_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue808-0018:04F3:319B-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue808-0018:04F3:319B-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_256C_006D_000F_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:256C:006D.000F.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:256C:006D.000F.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_5110_0003_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:5110.0003.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:5110.0003.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_4004_0003_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:4004.0003.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:4004.0003.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_25BF_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:25BF.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:25BF.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue615_0003_0B05_19B6_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue615-0003:0B05:19B6-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue615-0003:0B05:19B6-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_29F5_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:29F5.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:29F5.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue820_0018_06CB_CDEB_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue820-0018:06CB:CDEB-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue820-0018:06CB:CDEB-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_2339_0004_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:2339.0004.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:2339.0004.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_256C_006D_0008_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:256C:006D.0008.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:256C:006D.0008.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_5215_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:5215.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:5215.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_5094_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:5094.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:5094.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_045E_0745_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:045E:0745.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:045E:0745.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_04F3_227A_0006_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:04F3:227A.0006.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:04F3:227A.0006.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue805_0018_056A_511B_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue805-0018:056A:511B-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue805-0018:056A:511B-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_48F6_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:48F6.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:48F6.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_0114_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:0114.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:0114.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_49C8_0005_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:49C8.0005.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:49C8.0005.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_51F3_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:51F3.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:51F3.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_52EE_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:52EE.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:52EE.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_52E8_0005_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:52E8.0005.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:52E8.0005.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_51FE_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:51FE.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:51FE.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_29D2_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:29D2.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:29D2.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_51AF_0003_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:51AF.0003.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:51AF.0003.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_5221_0003_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:5221.0003.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:5221.0003.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_50EF_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:50EF.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:50EF.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue766_0003_056A_034F_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue766-0003:056A:034F-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue766-0003:056A:034F-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_4957_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:4957.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:4957.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_5048_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:5048.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:5048.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_51B2_0006_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:51B2.0006.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:51B2.0006.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_490A_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:490A.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:490A.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_5309_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:5309.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:5309.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_48ED_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:48ED.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:48ED.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue489_0018_06CB_CD73_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue489-0018:06CB:CD73-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue489-0018:06CB:CD73-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_037A_0014_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:037A.0014.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:037A.0014.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_2AF1_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:2AF1.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:2AF1.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_509F_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:509F.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:509F.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue875_0018_06CB_CD73_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue875-0018:06CB:CD73-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue875-0018:06CB:CD73-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_48B7_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:48B7.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:48B7.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_256C_5101_0003_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:256C:5101.0003.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:256C:5101.0003.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_28BD_0905_0033_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:28BD:0905.0033.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:28BD:0905.0033.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_2AD8_0005_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:2AD8.0005.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:2AD8.0005.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue524_0005_004C_0265_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue524-0005:004C:0265-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue524-0005:004C:0265-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_014E_0007_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:014E.0007.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:014E.0007.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue380_0018_06CB_82F5_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue380-0018:06CB:82F5-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue380-0018:06CB:82F5-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_5203_0006_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:5203.0006.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:5203.0006.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue503_0018_06CB_76B1_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue503-0018:06CB:76B1-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue503-0018:06CB:76B1-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_5157_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:5157.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:5157.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_5019_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:5019.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:5019.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue482_0003_046D_C05A_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue482-0003:046D:C05A-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue482-0003:046D:C05A-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue977_0003_1532_008A_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue977-0003:1532:008A-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue977-0003:1532:008A-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue948_0018_056A_52C2_1_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue948-0018:056A:52C2-1.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue948-0018:056A:52C2-1.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue789_0003_256C_006D_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue789-0003:256C:006D-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue789-0003:256C:006D-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_2D1F_524C_0004_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:2D1F:524C.0004.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:2D1F:524C.0004.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_03A6_0005_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:03A6.0005.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:03A6.0005.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_014D_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:014D.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:014D.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_256C_0064_0006_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:256C:0064.0006.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:256C:0064.0006.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_5202_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:5202.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:5202.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_3140_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:3140.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:3140.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_51C4_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:51C4.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:51C4.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue437_0018_04F3_3115_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue437-0018:04F3:3115-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue437-0018:04F3:3115-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_3041_0003_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:3041.0003.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:3041.0003.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue557_0018_04F3_311C_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue557-0018:04F3:311C-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue557-0018:04F3:311C-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_256C_006E_000E_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:256C:006E.000E.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:256C:006E.000E.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_256C_0066_0016_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:256C:0066.0016.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:256C:0066.0016.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_03EC_0004_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:03EC.0004.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:03EC.0004.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue908_0003_256C_006E_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue908-0003:256C:006E-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue908-0003:256C:006E-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_056a_00e3_1_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/056a:00e3_1.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 056a:00e3_1.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_4921_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:4921.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:4921.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue674_0018_0911_5288_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue674-0018:0911:5288-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue674-0018:0911:5288-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_256C_006D_000D_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:256C:006D.000D.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:256C:006D.000D.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_3231_0004_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:3231.0004.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:3231.0004.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_28BD_0936_0003_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:28BD:0936.0003.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:28BD:0936.0003.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_06CB_125D_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:06CB:125D.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:06CB:125D.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_31FB_0009_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:31FB.0009.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:31FB.0009.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_045E_09B5_0003_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:045E:09B5.0003.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:045E:09B5.0003.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_5008_0004_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:5008.0004.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:5008.0004.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_51AF_0005_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:51AF.0005.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:51AF.0005.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue971_0018_0488_120A_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue971-0018:0488:120A-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue971-0018:0488:120A-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_5155_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:5155.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:5155.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue925_0018_2D1F_0040_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue925-0018:2D1F:0040-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue925-0018:2D1F:0040-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_3202_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:3202.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:3202.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_5093_0004_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:5093.0004.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:5093.0004.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_00E3_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:00E3.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:00E3.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_5216_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:5216.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:5216.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_28BD_0935_0008_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:28BD:0935.0008.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:28BD:0935.0008.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_056a_0093_0_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/056a:0093_0.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 056a:0093_0.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_28BD_0935_0009_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:28BD:0935.0009.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:28BD:0935.0009.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_5147_0006_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:5147.0006.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:5147.0006.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_045E_09B5_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:045E:09B5.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:045E:09B5.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_30C1_0009_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:30C1.0009.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:30C1.0009.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_03D0_0005_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:03D0.0005.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:03D0.0005.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_0374_0005_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:0374.0005.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:0374.0005.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue382_0018_06CB_7A13_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue382-0018:06CB:7A13-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue382-0018:06CB:7A13-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_49BC_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:49BC.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:49BC.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_3193_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:3193.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:3193.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_27C6_0113_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:27C6:0113.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:27C6:0113.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue201_0003_1FD1_03EA_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue201-0003:1FD1:03EA-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue201-0003:1FD1:03EA-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_4846_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:4846.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:4846.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue701_0005_045E_02FD_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue701-0005:045E:02FD-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue701-0005:045E:02FD-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0019_045E_09AE_0003_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0019:045E:09AE.0003.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0019:045E:09AE.0003.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_03C4_0011_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:03C4.0011.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:03C4.0011.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue852_0003_046D_C091_1_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue852-0003:046D:C091-1.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue852-0003:046D:C091-1.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_30E3_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:30E3.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:30E3.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue884_0003_258A_0027_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue884-0003:258A:0027-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue884-0003:258A:0027-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_256C_006E_000D_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:256C:006E.000D.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:256C:006E.000D.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue339_0005_05AC_030E_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue339-0005:05AC:030E-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue339-0005:05AC:030E-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_4900_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:4900.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:4900.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_48CC_0007_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:48CC.0007.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:48CC.0007.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue987_0003_256C_006B_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue987-0003:256C:006B-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue987-0003:256C:006B-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue823_0018_04F3_2CF9_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue823-0018:04F3:2CF9-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue823-0018:04F3:2CF9-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue602_0018_04F3_3138_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue602-0018:04F3:3138-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue602-0018:04F3:3138-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue341_0018_056A_511A_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue341-0018:056A:511A-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue341-0018:056A:511A-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_045E_00DB_0003_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:045E:00DB.0003.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:045E:00DB.0003.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_28BD_0933_0017_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:28BD:0933.0017.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:28BD:0933.0017.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_056a_5090_0_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/056a:5090_0.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 056a:5090_0.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_31A5_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:31A5.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:31A5.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue613_0003_046D_C08B_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue613-0003:046D:C08B-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue613-0003:046D:C08B-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue449_0018_04F3_30D0_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue449-0018:04F3:30D0-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue449-0018:04F3:30D0-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_256C_006D_0007_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:256C:006D.0007.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:256C:006D.0007.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_2B5F_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:2B5F.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:2B5F.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_3057_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:3057.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:3057.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_5159_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:5159.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:5159.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_00F6_1_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:00F6_1.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:00F6_1.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_5222_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:5222.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:5222.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_4A0A_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:4A0A.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:4A0A.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_51B3_0003_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:51B3.0003.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:51B3.0003.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_256C_006D_0022_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:256C:006D.0022.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:256C:006D.0022.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue907_0018_06CB_CE57_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue907-0018:06CB:CE57-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue907-0018:06CB:CE57-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_037A_0013_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:037A.0013.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:037A.0013.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue520_0003_16D0_09A0_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue520-0003:16D0:09A0-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue520-0003:16D0:09A0-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_256C_0064_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:256C:0064.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:256C:0064.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_315B_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:315B.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:315B.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_3134_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:3134.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:3134.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_28BD_091F_001A_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:28BD:091F.001A.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:28BD:091F.001A.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_51D0_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:51D0.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:51D0.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_2718_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:2718.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:2718.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_2C86_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:2C86.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:2C86.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue575_0018_06CB_7F28_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue575-0018:06CB:7F28-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue575-0018:06CB:7F28-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_2817_0005_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:2817.0005.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:2817.0005.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue850_0003_056A_037B_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue850-0003:056A:037B-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue850-0003:056A:037B-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue468_0003_258A_001E_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue468-0003:258A:001E-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue468-0003:258A:001E-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue572_0018_04F3_3124_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue572-0018:04F3:3124-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue572-0018:04F3:3124-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_2D1F_001E_0007_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:2D1F:001E.0007.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:2D1F:001E.0007.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_306A_0006_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:306A.0006.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:306A.0006.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_30E4_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:30E4.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:30E4.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue915_0018_27C6_01E0_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue915-0018:27C6:01E0-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue915-0018:27C6:01E0-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue554_0018_04F3_304B_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue554-0018:04F3:304B-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue554-0018:04F3:304B-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue828_0003_5543_0081_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue828-0003:5543:0081-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue828-0003:5543:0081-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_52B0_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:52B0.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:52B0.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue896_0018_04F3_31B9_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue896-0018:04F3:31B9-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue896-0018:04F3:31B9-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_03eb_8c15_0005_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:03eb:8c15.0005.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:03eb:8c15.0005.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_045E_0922_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:045E:0922.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:045E:0922.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_056a_00ec_0_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/056a:00ec_0.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 056a:00ec_0.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_504C_0006_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:504C.0006.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:504C.0006.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_056a_5013_0_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/056a:5013_0.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 056a:5013_0.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_28BD_0934_0009_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:28BD:0934.0009.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:28BD:0934.0009.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue672_0003_0416_C168_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue672-0003:0416:C168-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue672-0003:0416:C168-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue811_0005_05AC_030E_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue811-0005:05AC:030E-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue811-0005:05AC:030E-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_2537_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:2537.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:2537.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue8530018_2D1F_0095_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue8530018:2D1F:0095-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue8530018:2D1F:0095-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_2522_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:2522.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:2522.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_0325_0005_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:0325.0005.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:0325.0005.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_51AF_0004_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:51AF.0004.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:51AF.0004.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_5220_0003_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:5220.0003.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:5220.0003.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue627_0003_17EF_60FB_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue627-0003:17EF:60FB-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue627-0003:17EF:60FB-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_24B8_0003_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:24B8.0003.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:24B8.0003.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_51BD_0004_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:51BD.0004.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:51BD.0004.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_256C_006D_0011_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:256C:006D.0011.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:256C:006D.0011.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_4900_0007_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:4900.0007.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:4900.0007.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_045E_0024_0004_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:045E:0024.0004.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:045E:0024.0004.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_0317_000B_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:0317.000B.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:0317.000B.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_51BE_0005_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:51BE.0005.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:51BE.0005.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue955_0003_25A7_FA7C_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue955-0003:25A7:FA7C-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue955-0003:25A7:FA7C-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_28BD_091F_001C_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:28BD:091F.001C.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:28BD:091F.001C.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_04F3_003D_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:04F3:003D.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:04F3:003D.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue486_0018_27C6_0111_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue486-0018:27C6:0111-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue486-0018:27C6:0111-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_5044_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:5044.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:5044.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue696_0003_046D_C077_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue696-0003:046D:C077-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue696-0003:046D:C077-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue510_0005_057E_0306_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue510-0005:057E:0306-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue510-0005:057E:0306-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_5048_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:5048.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:5048.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue641_0018_06CB_CD8B_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue641-0018:06CB:CD8B-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue641-0018:06CB:CD8B-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0019_045E_0922_0007_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0019:045E:0922.0007.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0019:045E:0922.0007.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue638_0018_27C6_0D42_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue638-0018:27C6:0D42-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue638-0018:27C6:0D42-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_5077_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:5077.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:5077.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue663_0018_06CB_CE39_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue663-0018:06CB:CE39-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue663-0018:06CB:CE39-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_251C_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:251C.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:251C.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_28BD_0934_0007_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:28BD:0934.0007.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:28BD:0934.0007.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_056a_00e2_0_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/056a:00e2_0.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 056a:00e2_0.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue432_0018_06CB_76AF_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue432-0018:06CB:76AF-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue432-0018:06CB:76AF-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue587_0018_06CB_8251_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue587-0018:06CB:8251-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue587-0018:06CB:8251-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_256C_006E_0008_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:256C:006E.0008.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:256C:006E.0008.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue650_0018_04F3_304A_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue650-0018:04F3:304A-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue650-0018:04F3:304A-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_4804_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:4804.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:4804.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_03D0_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:03D0.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:03D0.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_27C6_0113_000C_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:27C6:0113.000C.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:27C6:0113.000C.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_4841_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:4841.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:4841.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_0157_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:0157.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:0157.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_2846_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:2846.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:2846.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_5174_0003_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:5174.0003.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:5174.0003.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue489_0003_1532_0233_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue489-0003:1532:0233-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue489-0003:1532:0233-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_27C6_0118_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:27C6:0118.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:27C6:0118.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue686_0003_03EB_8A6E_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue686-0003:03EB:8A6E-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue686-0003:03EB:8A6E-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_0010_0004_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:0010.0004.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:0010.0004.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_50B6_0008_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:50B6.0008.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:50B6.0008.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_4995_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:4995.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:4995.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_2D1F_0163_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:2D1F:0163.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:2D1F:0163.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue450_0018_06CB_7A13_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue450-0018:06CB:7A13-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue450-0018:06CB:7A13-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue638_0018_1FD2_8008_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue638-0018:1FD2:8008-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue638-0018:1FD2:8008-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_49A0_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:49A0.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:49A0.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_5146_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:5146.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:5146.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_526C_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:526C.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:526C.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_256C_006D_0004_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:256C:006D.0004.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:256C:006D.0004.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_425A_0008_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:425A.0008.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:425A.0008.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0001_045E_0C1B_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0001:045E:0C1B.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0001:045E:0C1B.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_2AF7_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:2AF7.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:2AF7.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue666_0003_1532_0032_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue666-0003:1532:0032-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue666-0003:1532:0032-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue531_0018_0488_121F_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue531-0018:0488:121F-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue531-0018:0488:121F-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_51B8_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:51B8.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:51B8.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue754_0003_04F3_2012_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue754-0003:04F3:2012-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue754-0003:04F3:2012-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue636_0018_04F3_311C_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue636-0018:04F3:311C-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue636-0018:04F3:311C-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_5040_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:5040.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:5040.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_52CD_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:52CD.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:52CD.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_03F0_000D_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:03F0.000D.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:03F0.000D.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_241E_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:241E.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:241E.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_03CB_000F_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:03CB.000F.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:03CB.000F.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue528_0003_046D_4082_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue528-0003:046D:4082-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue528-0003:046D:4082-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_03F0_000B_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:03F0.000B.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:03F0.000B.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_256C_0066_0017_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:256C:0066.0017.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:256C:0066.0017.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_494A_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:494A.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:494A.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_0319_0004_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:0319.0004.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:0319.0004.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue261_0018_04F3_2812_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue261-0018:04F3:2812-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue261-0018:04F3:2812-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue765_0003_046D_4071_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue765-0003:046D:4071-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue765-0003:046D:4071-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_28BD_0934_0008_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:28BD:0934.0008.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:28BD:0934.0008.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_2A1C_0005_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:2A1C.0005.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:2A1C.0005.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue562_0018_06CB_CE37_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue562-0018:06CB:CE37-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue562-0018:06CB:CE37-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_256C_006D_0009_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:256C:006D.0009.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:256C:006D.0009.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_52E1_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:52E1.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:52E1.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_51BE_0006_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:51BE.0006.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:51BE.0006.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue946_0018_2808_0101_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue946-0018:2808:0101-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue946-0018:2808:0101-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_51F9_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:51F9.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:51F9.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_256C_0064_0005_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:256C:0064.0005.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:256C:0064.0005.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_5040_0004_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:5040.0004.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:5040.0004.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_28BD_0905_0031_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:28BD:0905.0031.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:28BD:0905.0031.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_5122_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:5122.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:5122.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_5128_0007_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:5128.0007.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:5128.0007.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_0043_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:0043.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:0043.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_037A_000F_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:037A.000F.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:037A.000F.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_2D1F_0012_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:2D1F:0012.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:2D1F:0012.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_00DE_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:00DE.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:00DE.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue243_0003_05AC_0265_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue243-0003:05AC:0265-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue243-0003:05AC:0265-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_3142_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:3142.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:3142.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue613_0003_04D9_1400_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue613-0003:04D9:1400-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue613-0003:04D9:1400-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_256C_0064_0004_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:256C:0064.0004.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:256C:0064.0004.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_5269_0003_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:5269.0003.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:5269.0003.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_48EE_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:48EE.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:48EE.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue834_0003_056A_0018_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue834-0003:056A:0018-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue834-0003:056A:0018-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_03CB_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:03CB.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:03CB.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue681_0003_046D_4072_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue681-0003:046D:4072-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue681-0003:046D:4072-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_256C_006B_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:256C:006B.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:256C:006B.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue728_0003_046D_406D_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue728-0003:046D:406D-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue728-0003:046D:406D-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_28BD_0936_0004_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:28BD:0936.0004.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:28BD:0936.0004.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue741_0018_056A_4808_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue741-0018:056A:4808-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue741-0018:056A:4808-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_0084_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:0084.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:0084.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_256C_006B_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:256C:006B.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:256C:006B.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue913_0003_17EF_60D0_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue913-0003:17EF:60D0-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue913-0003:17EF:60D0-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue600_0003_256C_006D_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue600-0003:256C:006D-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue600-0003:256C:006D-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_3044_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:3044.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:3044.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_0350_000B_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:0350.000B.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:0350.000B.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_2AD9_000B_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:2AD9.000B.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:2AD9.000B.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue961_0018_04F3_2859_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue961-0018:04F3:2859-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue961-0018:04F3:2859-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue467_0003_1BCF_08A6_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue467-0003:1BCF:08A6-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue467-0003:1BCF:08A6-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_3147_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:3147.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:3147.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_23DB_0006_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:23DB.0006.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:23DB.0006.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue708_0018_06CB_7A13_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue708-0018:06CB:7A13-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue708-0018:06CB:7A13-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_00E6_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:00E6.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:00E6.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue417_0018_06CB_CD7A_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue417-0018:06CB:CD7A-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue417-0018:06CB:CD7A-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_056a_010e_0_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/056a:010e_0.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 056a:010e_0.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_0060_0021_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:0060.0021.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:0060.0021.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_5093_0003_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:5093.0003.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:5093.0003.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_3136_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:3136.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:3136.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_056a_0090_0_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/056a:0090_0.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 056a:0090_0.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_535F_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:535F.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:535F.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_033C_000B_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:033C.000B.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:033C.000B.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue758_0018_06CB_CD41_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue758-0018:06CB:CD41-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue758-0018:06CB:CD41-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_045E_0040_0004_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:045E:0040.0004.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:045E:0040.0004.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue959_0018_06CB_76AF_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue959-0018:06CB:76AF-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue959-0018:06CB:76AF-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_256C_006D_0005_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:256C:006D.0005.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:256C:006D.0005.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_03CB_000D_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:03CB.000D.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:03CB.000D.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_22CA_0005_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:22CA.0005.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:22CA.0005.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue658_0003_2D1F_524C_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue658-0003:2D1F:524C-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue658-0003:2D1F:524C-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue329_0018_044E_120B_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue329-0018:044E:120B-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue329-0018:044E:120B-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue366_0003_046D_4082_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue366-0003:046D:4082-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue366-0003:046D:4082-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_4807_0008_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:4807.0008.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:4807.0008.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_03CB_000A_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:03CB.000A.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:03CB.000A.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue423_0018_0488_121F_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue423-0018:0488:121F-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue423-0018:0488:121F-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_256C_0064_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:256C:0064.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:256C:0064.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue575_0003_048D_C100_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue575-0003:048D:C100-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue575-0003:048D:C100-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_5146_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:5146.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:5146.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_509C_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:509C.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:509C.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_256C_006D_000E_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:256C:006D.000E.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:256C:006D.000E.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_2E2D_0006_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:2E2D.0006.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:2E2D.0006.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue545_0018_04F3_311C_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue545-0018:04F3:311C-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue545-0018:04F3:311C-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_524D_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:524D.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:524D.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_00F8_0_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:00F8_0.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:00F8_0.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_045E_07B2_0003_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:045E:07B2.0003.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:045E:07B2.0003.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_29A1_0004_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:29A1.0004.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:29A1.0004.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_0148_0009_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:0148.0009.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:0148.0009.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue616_0003_04D9_A09F_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue616-0003:04D9:A09F-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue616-0003:04D9:A09F-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_03CE_008D_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:03CE.008D.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:03CE.008D.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue682_0018_04F3_2436_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue682-0018:04F3:2436-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue682-0018:04F3:2436-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue967_0018_0488_104B_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue967-0018:0488:104B-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue967-0018:0488:104B-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_045E_09B5_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:045E:09B5.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:045E:09B5.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_50E9_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:50E9.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:50E9.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_485E_0005_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:485E.0005.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:485E.0005.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue996_0005_046D_B034_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue996-0005:046D:B034-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue996-0005:046D:B034-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_2AF4_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:2AF4.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:2AF4.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_3187_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:3187.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:3187.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_2283_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:2283.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:2283.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue410_0003_1532_0224_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue410-0003:1532:0224-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue410-0003:1532:0224-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue932_0003_04D9_0407_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue932-0003:04D9:0407-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue932-0003:04D9:0407-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue944_0003_062A_2901_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue944-0003:062A:2901-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue944-0003:062A:2901-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_0061_0022_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:0061.0022.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:0061.0022.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_28BD_0928_0008_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:28BD:0928.0008.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:28BD:0928.0008.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_045E_07B2_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:045E:07B2.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:045E:07B2.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_5099_0005_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:5099.0005.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:5099.0005.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_5218_0003_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:5218.0003.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:5218.0003.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_045E_07DF_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:045E:07DF.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:045E:07DF.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_4838_0004_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:4838.0004.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:4838.0004.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue869_0018_1B96_1B05_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue869-0018:1B96:1B05-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue869-0018:1B96:1B05-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_24DB_0005_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:24DB.0005.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:24DB.0005.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_51BF_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:51BF.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:51BF.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_5349_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:5349.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:5349.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_056a_00e6_0_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/056a:00e6_0.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 056a:00e6_0.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue438_0003_17EF_60B5_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue438-0003:17EF:60B5-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue438-0003:17EF:60B5-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_03ED_0009_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:03ED.0009.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:03ED.0009.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_28BD_094B_0002_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:28BD:094B.0002.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:28BD:094B.0002.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_056A_4870_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:056A:4870.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:056A:4870.0001.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0018_04F3_2B0A_0005_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0018:04F3:2B0A.0005.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0018:04F3:2B0A.0005.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_libinput_issue435_0018_27C6_01F0_0_rdesc() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/libinput-issue435-0018:27C6:01F0-0.rdesc").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse libinput-issue435-0018:27C6:01F0-0.rdesc"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_03CE_0004_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:03CE.0004.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:03CE.0004.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_056a_0117_1_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/056a:0117_1.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 056a:0117_1.hid.bin"));
        }
    }
    

    #[test]
    #[allow(non_snake_case)]
    fn test_0003_056A_5155_0001_hid_bin() {
        let bytes: Vec<u8> = std::fs::read("/home/dbydd/.cargo/git/checkouts/hidreport-nostd-1baa71420a9250bb/0d1f7c6/tests/data/0003:056A:5155.0001.hid.bin").unwrap();
        if !bytes.is_empty() {
            ReportDescriptor::try_from(&bytes).expect(&format!("Failed to parse 0003:056A:5155.0001.hid.bin"));
        }
    }
    
