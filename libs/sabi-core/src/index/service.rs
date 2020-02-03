/// see also:
///  * [AWS Service Endpoints - AWS General Reference](https://docs.aws.amazon.com/general/latest/gr/rande.html)
///
#[derive(Debug)]
pub enum ServiceCode {
    Iam,
    S3,
    Unknown(String),
}

impl ServiceCode {
    pub fn as_str(&self) -> &str {
        match self {
            ServiceCode::Iam => "iam",
            ServiceCode::S3 => "s3",
            ServiceCode::Unknown(code) => &code,
        }
    }
}

impl<'a> Into<&'a [u8]> for &'a ServiceCode {
    fn into(self) -> &'a [u8] {
        self.as_str().as_bytes()
    }
}