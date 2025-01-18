use aok::Result;
use pb::Func;
use pb_jelly::Message;
pub struct Hpc;

impl hpc::Hpc for Hpc {
  type Func = Func;
  async fn run(func: Func, args: impl AsRef<[u8]>) -> Result<Vec<u8>> {
    let args = args.as_ref();
    Ok(match func {
      Func::None => return Ok(vec![]),
      Func::Captcha => {
        let r = captcha::captcha().await?;
        pb::captcha::Captcha {
          id: r.id.into(),
          img: r.img.into(),
          tip: r.tip.into(),
        }
        .serialize_to_vec()
      }
      Func::AuthSignupMail => {
        let args = pb::auth::SignupMailArgs::deserialize_from_slice(args)?;
        auth::signup_mail(&args.address, &args.password).await?;
        vec![]
      }
      Func::AuthSignupMailVerify => todo!(),
    })
  }
}
