/**
 * Library WASM
 * 
 * Library ini digunakan untuk melakukan encoding dan decoding
 * menggunakan algoritma Caesar Chiper.
 * 
 */


use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn caesar_encode(text: &str, shift: u8) -> String {
    text.chars()
        .map(|character| {
            if character.is_ascii_alphabetic() {
                let first = if character.is_ascii_lowercase() {
                    // prefix b adalah byte squence
                    // untuk yang bertipe lower case
                    b'a'
                } else {
                    b'A'
                };

                // return berupa nilai char hasil perhitungan adalah angka
                // contoh:
                // print!("{}", (97 as char)); output = a
                // print!("{}", (97 as char)); output = A
                (first + ((character as u8 + shift - first) % 26)) as char
            } else {
                // jika bukan alphabet return karakter aslinya
                character
            }
        })
        .collect()
}

#[wasm_bindgen]
pub fn caesar_decode(text: &str, shift: u8) -> String {
    text.chars()
        .map(|character| {
            if character.is_ascii_alphabetic() {
                let first = if character.is_ascii_lowercase() {
                    // prefix b adalah byte squence
                    // untuk yang bertipe lower case
                    b'a'
                } else {
                    // untuk yang bertipe upper case
                    b'A'
                };

                // return berupa nilai char hasil perhitungan adalah angka
                // contoh:
                // print!("{}", (97 as char)); output = a
                // print!("{}", (65 as char)); output = A
                let exec = (character as u8) - (shift);

                // kode percabangan dibawah bertujuan
                // untuk mengembalikan ke nilai aslinya ketika kararakter
                // melewati proses perputaran
                // misal shift 5 dengan karakter w maka ketika diencode 
                // akan menjadi b maka  setelah pengurangan dengan shift 
                // perlu untuk menambah dengan 26

                // Lowercase
                if first == 97 {
                    if exec < 97 {
                        return (exec + 26) as char;
                    }
                }

                // Uppsercase
                if first == 65 {
                    if exec < 65 {
                        return (exec + 26) as char;
                    }
                }
                exec as char
            } else {
                // jika bukan alphabet return karakter aslinya
                character
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_encode() {
        assert_eq!(caesar_encode("", 13), "");
    }

    #[test]
    fn caesar_rot_13_encode() {
        assert_eq!(caesar_encode("Rust", 13), "Ehfg");
    }

    #[test]
    fn caesar_unicode_encode() {
        assert_eq!(caesar_encode("attack at dawn 攻", 5), "fyyfhp fy ifbs 攻");
    }

    #[test]
    fn empty_decode() {
        assert_eq!(caesar_decode("", 13), "");
    }

    #[test]
    fn caesar_rot_13_decode() {
        assert_eq!(caesar_decode("Ehfg", 13), "Rust");
    }

    #[test]
    fn caesar_unicode_decode() {
        assert_eq!(caesar_decode("fyyfhp fy ifbs 攻", 5), "attack at dawn 攻");
    }

    #[test]
    fn caesar_encode_lorem() {
        assert_eq!(caesar_encode("Lorem ipsum dolor sit amet, consectetur adipiscing elit. Maecenas ultrices et magna in finibus. Phasellus consequat porttitor nunc id laoreet. Interdum et malesuada fames ac ante ipsum primis in faucibus. Nullam maximus, ligula sit amet hendrerit malesuada, ante sapien egestas mi, ac egestas nibh quam in sem. Ut vitae ipsum dui. Etiam eu metus quam. Sed quis neque dolor. Morbi sit amet ipsum sed purus sagittis mattis vitae sed lectus.

        Praesent id bibendum augue, ut interdum dui. Proin et iaculis metus, ac efficitur ligula. Nullam nisl nulla, sodales quis nisi vitae, eleifend consectetur elit. Maecenas a lacus mauris. Donec sed quam nec magna ultricies tempus elementum imperdiet eros. Praesent in bibendum sem. Donec viverra, dui id fermentum viverra, ante nibh eleifend risus, sed commodo magna augue auctor nisi. Nulla facilisi. Phasellus vel maximus nisi. Proin dapibus, nisi eu venenatis lacinia, libero sem sollicitudin ante, a viverra orci orci quis augue. Proin ante augue, sagittis id quam in, imperdiet ultrices nisi. Vivamus facilisis libero eget mi porta, vel scelerisque sapien ultrices. Curabitur eleifend laoreet tellus ac tempor. Fusce gravida lacinia neque, sed pretium nisi pellentesque nec. Vivamus tempus magna vitae tristique blandit. Pellentesque aliquet et erat ac iaculis.
        
        Morbi quis enim scelerisque metus lobortis consectetur non nec nulla. Fusce dignissim ultrices est viverra tristique. Mauris in lorem est. Nulla porta id nibh in auctor. Aenean egestas viverra elementum. Suspendisse euismod, neque eget consequat lacinia, nulla augue mattis neque, et feugiat dolor metus sed arcu. In quis libero ullamcorper metus fringilla tempor a a metus. Suspendisse dapibus et tellus sit amet hendrerit. Vestibulum sollicitudin nisl ex. Sed blandit ligula id euismod malesuada. Praesent non pretium augue. In viverra iaculis aliquam. Nunc ac eros quis est vestibulum convallis et eu nisi.
        
        Pellentesque hendrerit sollicitudin metus sit amet sollicitudin. Proin vestibulum finibus malesuada. Donec sed turpis eros. Aenean quis egestas urna. Praesent porttitor quam eget tincidunt malesuada. Vivamus faucibus, lectus sit amet pellentesque lobortis, magna nibh pellentesque sapien, non dapibus odio sapien lacinia enim. Nam sodales dolor enim, eget semper nisi viverra sed. Aenean sed tortor tincidunt, sodales ante sit amet, sollicitudin mauris. Quisque blandit lacinia massa. Vestibulum tortor odio, feugiat id venenatis eget, gravida id nulla. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Aliquam et felis sit amet diam condimentum efficitur id ut purus. Fusce tempor viverra finibus. Sed facilisis, nibh et tristique ornare, metus dolor aliquet ante, a condimentum dolor neque non augue. Quisque lacinia nibh eget diam gravida, in posuere elit tempus.
        
        Etiam sollicitudin tristique molestie. Ut enim tellus, pellentesque viverra dolor eu, ullamcorper finibus lectus. Quisque consectetur, diam ut sollicitudin lacinia, diam tortor feugiat nisl, quis lobortis sem ligula vitae lectus. Aenean a dolor aliquet, pharetra nisi at, pulvinar nulla. Nam ullamcorper efficitur elit, in molestie mi finibus ut. Donec tincidunt, nulla gravida pharetra venenatis, ligula turpis placerat nulla, ullamcorper egestas nisi erat ut elit. Phasellus at massa quam. Donec volutpat arcu urna, eget lobortis erat tincidunt ut. Aenean dapibus urna vitae aliquam interdum. In hac habitasse platea dictumst. Duis laoreet lacus dignissim cursus rutrum. Quisque mattis sodales odio a tristique. Mauris sed dui odio."
        , 40), "Zcfsa wdgia rczcf gwh oash, qcbgsqhshif orwdwgqwbu szwh. Aosqsbog izhfwqsg sh aoubo wb twbwpig. Dvogszzig qcbgseioh dcfhhwhcf bibq wr zocfssh. Wbhsfria sh aozsgioro toasg oq obhs wdgia dfwawg wb toiqwpig. Bizzoa aolwaig, zwuizo gwh oash vsbrfsfwh aozsgioro, obhs godwsb susghog aw, oq susghog bwpv eioa wb gsa. Ih jwhos wdgia riw. Shwoa si ashig eioa. Gsr eiwg bseis rczcf. Acfpw gwh oash wdgia gsr difig gouwhhwg aohhwg jwhos gsr zsqhig.

        Dfosgsbh wr pwpsbria oiuis, ih wbhsfria riw. Dfcwb sh woqizwg ashig, oq sttwqwhif zwuizo. Bizzoa bwgz bizzo, gcrozsg eiwg bwgw jwhos, szswtsbr qcbgsqhshif szwh. Aosqsbog o zoqig aoifwg. Rcbsq gsr eioa bsq aoubo izhfwqwsg hsadig szsasbhia wadsfrwsh sfcg. Dfosgsbh wb pwpsbria gsa. Rcbsq jwjsffo, riw wr tsfasbhia jwjsffo, obhs bwpv szswtsbr fwgig, gsr qcaacrc aoubo oiuis oiqhcf bwgw. Bizzo toqwzwgw. Dvogszzig jsz aolwaig bwgw. Dfcwb rodwpig, bwgw si jsbsbohwg zoqwbwo, zwpsfc gsa gczzwqwhirwb obhs, o jwjsffo cfqw cfqw eiwg oiuis. Dfcwb obhs oiuis, gouwhhwg wr eioa wb, wadsfrwsh izhfwqsg bwgw. Jwjoaig toqwzwgwg zwpsfc sush aw dcfho, jsz gqszsfwgeis godwsb izhfwqsg. Qifopwhif szswtsbr zocfssh hszzig oq hsadcf. Tigqs ufojwro zoqwbwo bseis, gsr dfshwia bwgw dszzsbhsgeis bsq. Jwjoaig hsadig aoubo jwhos hfwghweis pzobrwh. Dszzsbhsgeis ozweish sh sfoh oq woqizwg.
        
        Acfpw eiwg sbwa gqszsfwgeis ashig zcpcfhwg qcbgsqhshif bcb bsq bizzo. Tigqs rwubwggwa izhfwqsg sgh jwjsffo hfwghweis. Aoifwg wb zcfsa sgh. Bizzo dcfho wr bwpv wb oiqhcf. Osbsob susghog jwjsffo szsasbhia. Gigdsbrwggs siwgacr, bseis sush qcbgseioh zoqwbwo, bizzo oiuis aohhwg bseis, sh tsiuwoh rczcf ashig gsr ofqi. Wb eiwg zwpsfc izzoaqcfdsf ashig tfwbuwzzo hsadcf o o ashig. Gigdsbrwggs rodwpig sh hszzig gwh oash vsbrfsfwh. Jsghwpizia gczzwqwhirwb bwgz sl. Gsr pzobrwh zwuizo wr siwgacr aozsgioro. Dfosgsbh bcb dfshwia oiuis. Wb jwjsffo woqizwg ozweioa. Bibq oq sfcg eiwg sgh jsghwpizia qcbjozzwg sh si bwgw.
        
        Dszzsbhsgeis vsbrfsfwh gczzwqwhirwb ashig gwh oash gczzwqwhirwb. Dfcwb jsghwpizia twbwpig aozsgioro. Rcbsq gsr hifdwg sfcg. Osbsob eiwg susghog ifbo. Dfosgsbh dcfhhwhcf eioa sush hwbqwribh aozsgioro. Jwjoaig toiqwpig, zsqhig gwh oash dszzsbhsgeis zcpcfhwg, aoubo bwpv dszzsbhsgeis godwsb, bcb rodwpig crwc godwsb zoqwbwo sbwa. Boa gcrozsg rczcf sbwa, sush gsadsf bwgw jwjsffo gsr. Osbsob gsr hcfhcf hwbqwribh, gcrozsg obhs gwh oash, gczzwqwhirwb aoifwg. Eiwgeis pzobrwh zoqwbwo aoggo. Jsghwpizia hcfhcf crwc, tsiuwoh wr jsbsbohwg sush, ufojwro wr bizzo. Cfqw jofwig bohceis dsbohwpig sh aoubwg rwg dofhifwsbh acbhsg, bogqshif fwrwqizig aig. Ozweioa sh tszwg gwh oash rwoa qcbrwasbhia sttwqwhif wr ih difig. Tigqs hsadcf jwjsffo twbwpig. Gsr toqwzwgwg, bwpv sh hfwghweis cfbofs, ashig rczcf ozweish obhs, o qcbrwasbhia rczcf bseis bcb oiuis. Eiwgeis zoqwbwo bwpv sush rwoa ufojwro, wb dcgisfs szwh hsadig.
        
        Shwoa gczzwqwhirwb hfwghweis aczsghws. Ih sbwa hszzig, dszzsbhsgeis jwjsffo rczcf si, izzoaqcfdsf twbwpig zsqhig. Eiwgeis qcbgsqhshif, rwoa ih gczzwqwhirwb zoqwbwo, rwoa hcfhcf tsiuwoh bwgz, eiwg zcpcfhwg gsa zwuizo jwhos zsqhig. Osbsob o rczcf ozweish, dvofshfo bwgw oh, dizjwbof bizzo. Boa izzoaqcfdsf sttwqwhif szwh, wb aczsghws aw twbwpig ih. Rcbsq hwbqwribh, bizzo ufojwro dvofshfo jsbsbohwg, zwuizo hifdwg dzoqsfoh bizzo, izzoaqcfdsf susghog bwgw sfoh ih szwh. Dvogszzig oh aoggo eioa. Rcbsq jczihdoh ofqi ifbo, sush zcpcfhwg sfoh hwbqwribh ih. Osbsob rodwpig ifbo jwhos ozweioa wbhsfria. Wb voq vopwhoggs dzohso rwqhiagh. Riwg zocfssh zoqig rwubwggwa qifgig fihfia. Eiwgeis aohhwg gcrozsg crwc o hfwghweis. Aoifwg gsr riw crwc.");
    }
}
