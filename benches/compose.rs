#![feature(test)]

extern crate test;

use httpencode::{HttpBuilder, Method, Version, Uri};

const REQ_SHORT: &'static [u8] = b"\
GET / HTTP/1.0\r\n\
Host: example.com\r\n\
Cookie: session=60; user_id=1\r\n\r\n";

const REQ: &'static [u8] = b"\
GET /wp-content/uploads/2010/03/hello-kitty-darth-vader-pink.jpg HTTP/1.1\r\n\
Host: www.kittyhell.com\r\n\
User-Agent: Mozilla/5.0 (Macintosh; U; Intel Mac OS X 10.6; ja-JP-mac; rv:1.9.2.3) Gecko/20100401 Firefox/3.6.3 Pathtraq/0.9\r\n\
Accept: text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8\r\n\
Accept-Language: ja,en-us;q=0.7,en;q=0.3\r\n\
Accept-Encoding: gzip,deflate\r\n\
Accept-Charset: Shift_JIS,utf-8;q=0.7,*;q=0.7\r\n\
Keep-Alive: 115\r\n\
Connection: keep-alive\r\n\
Cookie: wp_ozh_wsa_visits=2; wp_ozh_wsa_visit_lasttime=xxxxxxxxxx; __utma=xxxxxxxxx.xxxxxxxxxx.xxxxxxxxxx.xxxxxxxxxx.xxxxxxxxxx.x; __utmz=xxxxxxxxx.xxxxxxxxxx.x.x.utmccn=(referral)|utmcsr=reader.livedoor.com|utmcct=/reader/|utmcmd=referral|padding=under256\r\n\r\n";

#[bench]
fn build_req_long(b: &mut test::Bencher) {
    let mut buf = Vec::new();
    buf.reserve(1 << 14);

    b.iter(|| -> Result<_, _> {
        let mut req = HttpBuilder::request(
            &mut buf, 
            Method::Get,
            Version::Http11,
            Uri::new(b"/wp-content/uploads/2010/03/hello-kitty-darth-vader-pink.jpg")
        )?;

        req.header("Host", "www.kittyhell.com")?;
        req.header("User-Agent", "Mozilla/5.0 (Macintosh; U; Intel Mac OS X 10.6; ja-JP-mac; rv:1.9.2.3) Gecko/20100401 Firefox/3.6.3 Pathtraq/0.9")?;
        req.header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8")?;
        req.header("Accept-Language", "ja,en-us;q=0.7,en;q=0.3")?;
        req.header("Accept-Encoding", "gzip,deflate")?;
        req.header("Accept-Charset", "Shift_JIS,utf-8;q=0.7,*;q=0.7")?;
        req.header("Keep-Alive", "115")?;
        req.header("Connection", "keep-alive")?;
        req.header("Cookie", "wp_ozh_wsa_visits=2; wp_ozh_wsa_visit_lasttime=xxxxxxxxxx; __utma=xxxxxxxxx.xxxxxxxxxx.xxxxxxxxxx.xxxxxxxxxx.xxxxxxxxxx.x; __utmz=xxxxxxxxx.xxxxxxxxxx.x.x.utmccn=(referral)|utmcsr=reader.livedoor.com|utmcct=/reader/|utmcmd=referral|padding=under256")?;

        req.finish().map(|_| ())
    })
}

#[bench]
fn build_req_long_unsafe(b: &mut test::Bencher) {
    let mut buf = Vec::new();
    buf.reserve(1 << 14);

    b.iter(|| -> Result<_, _> {
        unsafe { 
            let mut req = HttpBuilder::request(
                &mut buf, 
                Method::Get,
                Version::Http11,
                Uri::escaped_unchecked(b"/wp-content/uploads/2010/03/hello-kitty-darth-vader-pink.jpg")
            )?;

            req.header_unchecked("Host", "www.kittyhell.com")?;
            req.header_unchecked("User-Agent", "Mozilla/5.0 (Macintosh; U; Intel Mac OS X 10.6; ja-JP-mac; rv:1.9.2.3) Gecko/20100401 Firefox/3.6.3 Pathtraq/0.9")?;
            req.header_unchecked("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8")?;
            req.header_unchecked("Accept-Language", "ja,en-us;q=0.7,en;q=0.3")?;
            req.header_unchecked("Accept-Encoding", "gzip,deflate")?;
            req.header_unchecked("Accept-Charset", "Shift_JIS,utf-8;q=0.7,*;q=0.7")?;
            req.header_unchecked("Keep-Alive", "115")?;
            req.header_unchecked("Connection", "keep-alive")?;
            req.header_unchecked("Cookie", "wp_ozh_wsa_visits=2; wp_ozh_wsa_visit_lasttime=xxxxxxxxxx; __utma=xxxxxxxxx.xxxxxxxxxx.xxxxxxxxxx.xxxxxxxxxx.xxxxxxxxxx.x; __utmz=xxxxxxxxx.xxxxxxxxxx.x.x.utmccn=(referral)|utmcsr=reader.livedoor.com|utmcct=/reader/|utmcmd=referral|padding=under256")?;

            req.finish().map(|_| ())
        }
    })
}

#[bench]
fn build_req_short(b: &mut test::Bencher) {
    let mut buf = Vec::new();
    buf.reserve(1 << 14);

    b.iter(|| -> Result<_, _> {
        let mut req = HttpBuilder::request(
            &mut buf, 
            Method::Get,
            Version::Http11,
            Uri::new(b"/")
        )?;

        req.header("Host", "example.com")?;
        req.header("Cookie", "session=60; user_id=1")?;

        req.finish().map(|_| ())
    })
}
