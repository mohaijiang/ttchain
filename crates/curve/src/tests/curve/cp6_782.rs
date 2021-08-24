use crate::{curve::CP6_782, CurveBasicOperations};

/// CP6_782 ADD
pub fn cp6_782_add() {
    // two one-points add encode
    let input1 = hex::decode(
            "0ec0176973190029442be14ed3d59b6b1c5edffbedf5b325e3fb4e8e5727390f66efebabe9aded0d53477fe7a8ced0db437cb6e580654119be0eeaa2f31a367d77f44a71c686fba448b83603ce76796e1518038ba606749b019a7571c78096d2dd0d0000000000002b42bf978df0e1d22d7a68c07530a308108f3553075f36664c3e9d38d2375cf59cc2f53f6071068df86635195fcd48f4e8ab36594b22a4111433ca6beb550372f11c2e640fc4e07a7b403ba124037fc81719ddd5aaf43a88f443a4735b09eaf5e813000000000000000ec0176973190029442be14ed3d59b6b1c5edffbedf5b325e3fb4e8e5727390f66efebabe9aded0d53477fe7a8ced0db437cb6e580654119be0eeaa2f31a367d77f44a71c686fba448b83603ce76796e1518038ba606749b019a7571c78096d2dd0d0000000000002b42bf978df0e1d22d7a68c07530a308108f3553075f36664c3e9d38d2375cf59cc2f53f6071068df86635195fcd48f4e8ab36594b22a4111433ca6beb550372f11c2e640fc4e07a7b403ba124037fc81719ddd5aaf43a88f443a4735b09eaf5e81300000000000000").unwrap();

    let res1 = CP6_782::add(&input1[..]).unwrap();

    let expected = hex::decode(
            "8a59e8337302d1f3a944462db081110246cfd825ce906197c9364af95a814ae6037909493d0393630581887019acd8c771040cdd0a13e44c6f0ebcb6c9a00c0aad9604e635acef8f8fdb7a348e91085053f3761f8f245e2a3f5c9621d1da78adbe160000000000002cf9422ee038952d966beba7a3aabeae5cf313be40a4f275e04f820c7c58c94900b9ff11f7584564761f40b431525239175742d2b9a01c64cc66a86cabc6b098594879203aea9f9c4d9ee5b74b9ea6c1c58084ab4564c153a15fe89ff5f8405cdd1700000000000000").unwrap();
    assert_eq!(res1, expected);
}

/// CP6_782 MUL
pub fn cp6_782_mul() {
    let input2 = hex::decode(
        "0ec0176973190029442be14ed3d59b6b1c5edffbedf5b325e3fb4e8e5727390f66efebabe9aded0d53477fe7a8ced0db437cb6e580654119be0eeaa2f31a367d77f44a71c686fba448b83603ce76796e1518038ba606749b019a7571c78096d2dd0d0000000000002b42bf978df0e1d22d7a68c07530a308108f3553075f36664c3e9d38d2375cf59cc2f53f6071068df86635195fcd48f4e8ab36594b22a4111433ca6beb550372f11c2e640fc4e07a7b403ba124037fc81719ddd5aaf43a88f443a4735b09eaf5e81300000000000000dab41652cc0635372e025c39eecbb41543853236e87cacff83bbc30aa775ee3a7f73cea5adfe6bef3969c6d7595e2f00").unwrap();

    let res2 = CP6_782::mul(&input2[..]).unwrap();

    let expected = hex::decode(
        "97169d2ea987e88ed3d56076fef75095d629a0259b91fdba98857a138f24f651b4e78bd8993934dc960f64c174fc75ad5d5e3dbda6cc646efd84790f284696fd936e452bc6565bd052c45cbc77b22ec102d6b63f7b9e0dadb17d6fb1a9ab6338f32700000000000060fe6c49d6b893273190c93cb3c0380826fe9ed6d878644b8e3eaca857698b09b6efc0dcba4b65b5a93ecdf3588d7e58039b386a3b6b69ea9e343a7152d5b4d88ff9c995cb330bfa97438a17971e78327e9c879e12e47c79500b6d1c096f4b0bc00c00000000000000").unwrap();

    assert_eq!(res2, expected);
}

/// CP6_782 MUL
pub fn cp6_782_pairing() {
    // vec![sa,b,-sb,a]
    let input = hex::decode(
                "63d6b88c9772a400d1d6c4bd8faa04ac061b45ffad95d99ea17d76b9fca649a0a84776fc793a71486eb8c7ca5b83d2636d587614b396595279449de429bc059228bf317f1d4da525cd6e44e848160f76a266a607a444a768afaf022b34d52d66972a000000000000d7e04fd04299104293da186fe28e96fe677eec50c4b2e408a0682281d97b78fd1aaeab4bf4eb630106e6279013310e8a194b46e7b0f97fa125bcaa81bcf699dce5de8ed3a04b830d8517105131c749db6dc4e5608c8780ef75aa445c378fc9c93926000000000000008f0b1ac76c6cb6c0a334a1bf12a15c5ea5e3151219096bf4cb708fd6966930d464160f39836e97bf55e5a2e666257ebf9ce8dc8b71729dd7fb3b69b983c379f263758b2872c061afa4fbfd24cb44402c8d5b6a78caaae7b84e9ec09dba705ebcf327000000000000b78c152b45da1525ff97d596227b242bcaeb1a9420a84412640015bd188d36f03b079aae242678adb0d1c56d247f99a6e25577abc570cb1d73789306fec3150a7e69895856f58f6f0a6155d8fd3d22ddcf5083acb95797127f2d05f6e97d0e9d981f000000000000bfac98f6be9c6035a3e51ca759983590bd7fe2b094d5d6506c69dfd6c0fc4094d3225b3aa38d61f366a080e73e94450dd07581b611307c4149965468ce945858bae85e3b502d4e00b5088c084ef3816eb7476056a173dc0d7182628b82dc5b61e90d000000000000325420227adb00fba5a336f9b3dbc771e62d0fb831643fbb89a153ab83226f26bc9cd25517ed3503838b13b54c1eb4a6c9b3e8d9d5c9d0868a50d37894344a4d41bc00cbaaed588312518d0dbe813d83f0f42d3e90ffc872a8212de98f2dab9d69240000000000008a57a6e71f72d19c90793c434cd77b0f5aa0f6a1eab20ae7cdcef0da9d1e21816d5e93761edd793e26a86329159df91882e95390084ecf5be2fba1c456c40edafae4aa8a64d02122442acb3d384ab7372f1d4d073b63aa3e2c84c563ee504773762700000000000094de4ee023b2f6ab6f6a52c2de5958a63993d224a410f75a7bdd74c84f6e379d2dfd9be248871192fcc5102fb9ea6e5914e5130f6f917e9d752ed425b8e9e30cf6cdf9229844d3f56db05463ad7f1f06e1dbc2e2df3f099bf9917c84b6aeb2f87c320000000000000097169d2ea987e88ed3d56076fef75095d629a0259b91fdba98857a138f24f651b4e78bd8993934dc960f64c174fc75ad5d5e3dbda6cc646efd84790f284696fd936e452bc6565bd052c45cbc77b22ec102d6b63f7b9e0dadb17d6fb1a9ab6338f327000000000000892c2732dfc03ab319b233310ec5244c9128ae882de3aea2bdc3176af20ca492837a3dc049e76f6576d4ced31e6e25fa8ebc43f78c2bccc0eca2bf0f3b9d57aa774338dc22f066708eace05cd4f8e489e5c9caf02a44dc6fcf888b8f32b786b9882b00000000000000a66fec711e683c264adcf7964ad7925227f1c379c6b70b855ad8f813336f0aa60df43eb06470a36e5e0d6981480ad56505d6495a343be33ad238d2776d97c4092d2f617601989f67e756d30f8ffc26f7a711d36fcf46101fae4ed34b614b7bf6930c000000000000936d18f2e311680f740ae3ca58e911149a131484ceaf26f1cebf3cd6d7f4e0e24be42c8a694882bce99a2c4a409a4196dd1fb446498531a00a714c3ba40d03936ee16976ea331f135f0f1481a7bc75be8bc0e19ae1a95387dbb14d27aea369291a14000000000000a86d8629bf6e0c009d7f5d0e50b7788803d2e3572869990baad5cc811966d7f2f089ff363bc63e9310e4790d0f4df201bbabbbe8de41e2dd7008a342f53171aa2165790f48bbefc1705abacd8ffeca9bc2ee9a6c6485bc8b6c935c45603959c4f63700000000000057175a1744fa9951e3c0f5a78f03f67a9bdd0b6d64fd14cd6719a838eb71679024fef7df151336559b5a3d10bdb5d35cb71f40712124e00af89b6285d76e2f11987bfa2d114d6610e37dd1e62f846091a2f4262c8c8fb336d51b012068ade19e902a000000000000772368ad3e515efe9189670d3cc3d35e351fc8f85570e5ec334f7d603527e33026e4df0b724318bb717e2a5097aa4175b32c692d1f5df5ada2a3dd5e4f82413da7f0a786c95da1e5f188dc03b1e409ba2fbf83965f750847be723d4f9904cdf906270000000000006470913a2bedd0ab9f52af14719548440be3bbbac5e9b4f2151bc4460a275df9f388e5309d5c3cd161e294425751a84af4e32cb7679f39b5f20e3f4f4f98af458aaeb36e86bff59b319bdd050cdbe88b0dcab9f9d342c5f41821ed83df1de8c07f1900000000000000").unwrap();

    // e(sa, b) = e(sb, a)
    // e(sa, b) * e(-sb, a) = 1
    assert!(CP6_782::pairings(&input[..]).expect("pairings failed"));
    // println!("test pairings{} success!", i + 1);
}

/// CP6_782 MUL
pub fn cp6_782_pairing_six() {
    // test pairings
    {
        cp6_782_pairing();
    }

    // check pairings
    {
        // hex![(a1, b1), (a2, b2), (-a1, b1), (-a2, b2)];
        let pairings_encoded = "0ec0176973190029442be14ed3d59b6b1c5edffbedf5b325e3fb4e8e5727390f66efebabe9aded0d53477fe7a8ced0db437cb6e580654119be0eeaa2f31a367d77f44a71c686fba448b83603ce76796e1518038ba606749b019a7571c78096d2dd0d0000000000002b42bf978df0e1d22d7a68c07530a308108f3553075f36664c3e9d38d2375cf59cc2f53f6071068df86635195fcd48f4e8ab36594b22a4111433ca6beb550372f11c2e640fc4e07a7b403ba124037fc81719ddd5aaf43a88f443a4735b09eaf5e813000000000000008a504ab3644e901e852ca8942c5d0a14b79eb7c040bea357f6cc187a0da7ebb33e4f04e052f9cb87440c46934c7c36f57ef065c3b780bc476938caf9c87dd4c2e35da0fa31756f15dbe8e8ab1953941a71d768b35101b994ec59752450ad6a60c821000000000000c2ecaf0b48c7435fdc25cffeef75758a2d2eea3e431f5aec9a637d2fa3d8db5cfbe38353e0665a2787059a624f0490c4689d3821775719d700eb1f6222406f02e2ed5ef7274d77a79f088fb15340e9fb300903ae9b3813f54c707791907528108233000000000000ec7001c6219c30a2fd217740a9a5d2aa39c5047243caf2cf010c466b74849c1e62bee8ead6029757d79d5e17821d93687bf8be7c350c28b075042b89395729d03125dd5492c75350b9e8812d3fdb78d5daa5555fa500b15faf6d5f3f0532884dd30900000000000093dc5cd7f3d516a1f18c45c974e5343ae16b2a3193948915dfec95d3874ff35f281a43e1f56278d1f315dc8e695621faaba04c56743c22307964df8a949acabb4e978b6c3efa56135736b92284b3468317791ef0bb67c60c7c443cff579c2f778e15000000000000889c75a94a040308c74bd6ed410915953ea32600eca23c4e5c065266566cb98e7f8cb5491cfd20fb64d62f0bab43391030424044fa983b3daad21034671907279c37f5f3a5ec436c8341957b27f1946cd939511559517b3472a4bed80112ebf3c9090000000000006124bd24fe662ad8fae7bf207c5e5446ecda7cd75a35111bf2dd7ba59e59eb189d67f67170e7c5b9afcd1ccaea422ec3d317238556892cd8719dddbf65aabd416c7be064648945c0953fc0e8bcf2a3c1d3e5838d214ce6e13b8dcd26cae6d236841b00000000000000a63b7bc10341614dfacc7e9db193fa858525277226eaa9f63d549a8167ffb197938dfe267f4dc00417d11706e808767e32c5bf061df779eafbd53afc8d046b988a0fc218fdf5ad00fbe269ca91d98f280ef417c446875eafb066c21d42c8073d6433000000000000e97e981e057a1f9c3acc8fdd3d5c023799c3cfc9306d80ca4d62b78ab4fc1da2bcb77e5c699f4b4e4279c6a45aa4c497720d7924f3ece540b425c8e7df30fb0b14800a48af97c682c13312e2fe3ff7387cda9c01c0f0a89fd9386441735f82104f26000000000000009574222721af4a64fb76392a7068f7e148582687ee50b42965e5494f91c6bd50c01badb90e6ececd9c667feaa73e1f30b4b0b96edd4c3410ef6c33aa5c3fd25946fdf3732402439e5d0765894a18d11a0f9b0b59352e21b3b5f991adfa99bc124d210000000000009f86b939ee2907f968c8ccadfc3698ae0a3d676a44c1bd6afdbd7d2050c5e7e8ab40bf5d9dec914aa08fc3c364d7b891a915d93924c29a239dfcaa75857aa9bf49b235b28cfd95bfd39d33c96d7b3d8db52d641e5e869273c6586f4d2fecfce9b02000000000000081b8163e6b0f6f2533d8636086ece848ba74aec8ae3a2c431ad80d9e2cf8a97d45010ae00f3f410050283764ad09b2eb71aa53da5ae78e25fb85c4d2375da56f93fd050d580af0559053684e42a1442df9e30ad2989a2c194def7cda006af0099930000000000000962ddad8a29b32221d25a4cf0b362d489540bc912335afc5b8bb30a4d805bd42f002ccfb6eaee9d0d232ba721c7022e30c061e6e10513fb9130ec5ad084e23736e8d97d7edda6a60725067b95c19ac3d08916be4ef3a960f163daef3204a019f1a0f0000000000008512c486050cfa97b6058ad7c8bd08dda3ce1310339d565d0082d1a45e612c458ecaf61f9e8bcc90472f8d3828fe5998cda1e8a6866eacc7321887501c569d5a8096b66e2470f37a7506b449e2cb29f86633f89c88ba20b09392fb6416fdcf34200e000000000000d0d2b6eab9b1e1c70835cd8f7a1a40c8b4dbb945697d8a21f0f6c69f251935a0ba9eceef183851e906ada02dcf246d191efa99f2388df5537483a1307cfeb7bce7f7b4102c2ffc7e6cbe7afa47c372e38735566e92a984e5a25632658c70ce5c3f36000000000000000ec0176973190029442be14ed3d59b6b1c5edffbedf5b325e3fb4e8e5727390f66efebabe9aded0d53477fe7a8ced0db437cb6e580654119be0eeaa2f31a367d77f44a71c686fba448b83603ce76796e1518038ba606749b019a7571c78096d2dd0d000000000000bee8d4e32789ec071dc894ad4b55ba4ba797170cfffcdc87ffc326da773ed3a69ca7085da4c1ce8d27ac66ae182e5b5ea9ab45087d74919977a42f15a21c09111620d40ddf5f91efaaaf2fd34614def34b4d75b992331e612b505438e01ce8ce5f24000000000000008a504ab3644e901e852ca8942c5d0a14b79eb7c040bea357f6cc187a0da7ebb33e4f04e052f9cb87440c46934c7c36f57ef065c3b780bc476938caf9c87dd4c2e35da0fa31756f15dbe8e8ab1953941a71d768b35101b994ec59752450ad6a60c821000000000000c2ecaf0b48c7435fdc25cffeef75758a2d2eea3e431f5aec9a637d2fa3d8db5cfbe38353e0665a2787059a624f0490c4689d3821775719d700eb1f6222406f02e2ed5ef7274d77a79f088fb15340e9fb300903ae9b3813f54c707791907528108233000000000000ec7001c6219c30a2fd217740a9a5d2aa39c5047243caf2cf010c466b74849c1e62bee8ead6029757d79d5e17821d93687bf8be7c350c28b075042b89395729d03125dd5492c75350b9e8812d3fdb78d5daa5555fa500b15faf6d5f3f0532884dd30900000000000093dc5cd7f3d516a1f18c45c974e5343ae16b2a3193948915dfec95d3874ff35f281a43e1f56278d1f315dc8e695621faaba04c56743c22307964df8a949acabb4e978b6c3efa56135736b92284b3468317791ef0bb67c60c7c443cff579c2f778e15000000000000889c75a94a040308c74bd6ed410915953ea32600eca23c4e5c065266566cb98e7f8cb5491cfd20fb64d62f0bab43391030424044fa983b3daad21034671907279c37f5f3a5ec436c8341957b27f1946cd939511559517b3472a4bed80112ebf3c9090000000000006124bd24fe662ad8fae7bf207c5e5446ecda7cd75a35111bf2dd7ba59e59eb189d67f67170e7c5b9afcd1ccaea422ec3d317238556892cd8719dddbf65aabd416c7be064648945c0953fc0e8bcf2a3c1d3e5838d214ce6e13b8dcd26cae6d236841b00000000000000a63b7bc10341614dfacc7e9db193fa858525277226eaa9f63d549a8167ffb197938dfe267f4dc00417d11706e808767e32c5bf061df779eafbd53afc8d046b988a0fc218fdf5ad00fbe269ca91d98f280ef417c446875eafb066c21d42c8073d643300000000000000acfb5cb0ffae3e10766d9083295b1d1e637d95d5ee9223fe9f0c88957911fa7cb27f409b9389ccdd99d5221d57dfba1f4a033dd5a94f6ad7b13199ad411177f3bcf7293f8cabe764bc58926cd76583e78bb58d7d37b049465b946ac8c64fb4f911000000000000009574222721af4a64fb76392a7068f7e148582687ee50b42965e5494f91c6bd50c01badb90e6ececd9c667feaa73e1f30b4b0b96edd4c3410ef6c33aa5c3fd25946fdf3732402439e5d0765894a18d11a0f9b0b59352e21b3b5f991adfa99bc124d210000000000009f86b939ee2907f968c8ccadfc3698ae0a3d676a44c1bd6afdbd7d2050c5e7e8ab40bf5d9dec914aa08fc3c364d7b891a915d93924c29a239dfcaa75857aa9bf49b235b28cfd95bfd39d33c96d7b3d8db52d641e5e869273c6586f4d2fecfce9b02000000000000081b8163e6b0f6f2533d8636086ece848ba74aec8ae3a2c431ad80d9e2cf8a97d45010ae00f3f410050283764ad09b2eb71aa53da5ae78e25fb85c4d2375da56f93fd050d580af0559053684e42a1442df9e30ad2989a2c194def7cda006af0099930000000000000962ddad8a29b32221d25a4cf0b362d489540bc912335afc5b8bb30a4d805bd42f002ccfb6eaee9d0d232ba721c7022e30c061e6e10513fb9130ec5ad084e23736e8d97d7edda6a60725067b95c19ac3d08916be4ef3a960f163daef3204a019f1a0f0000000000008512c486050cfa97b6058ad7c8bd08dda3ce1310339d565d0082d1a45e612c458ecaf61f9e8bcc90472f8d3828fe5998cda1e8a6866eacc7321887501c569d5a8096b66e2470f37a7506b449e2cb29f86633f89c88ba20b09392fb6416fdcf34200e000000000000d0d2b6eab9b1e1c70835cd8f7a1a40c8b4dbb945697d8a21f0f6c69f251935a0ba9eceef183851e906ada02dcf246d191efa99f2388df5537483a1307cfeb7bce7f7b4102c2ffc7e6cbe7afa47c372e38735566e92a984e5a25632658c70ce5c3f3600000000000000";

        let input = hex::decode(pairings_encoded).unwrap();

        // check pairings operation:(a1*b1) * e(a2*b2) * e(-a1*b1) * e(-a2*b2) == 1 return true
        assert!(CP6_782::pairings(&input[..]).unwrap());
        // println!("test pairings e(a1*b1)*e(a2*b2)*e(-a1*b1)*e(-a2*b2) success!");
    }
}

#[test]
fn test_cp6_782_additional() {
    // zero-points additions
    {
        let input = hex::decode(
            "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001").unwrap();

        let res = CP6_782::add(&input[..]).unwrap();

        let expected = hex::decode(
            "0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001").unwrap();

        assert_eq!(&expected[..], &res[..]);
        // println!("test add1 success!");
    }

    // one-points additions
    cp6_782_add();
}

#[test]
fn test_cp6_782_scalar_mul() {
    // one-point mul 2 encode
    cp6_782_mul();
}

#[test]
fn test_cp6_782_pairing() {
    for i in 0..5 {
        cp6_782_pairing_six();
    }
}