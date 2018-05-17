# rust-pubsub-example

This is a example of google-pubsub.

# dependencies

- Rust >= 1.26.0
- [hyper](https://github.com/hyperium/hyper)
- [yup-oauth2](https://github.com/dermesser/yup-oauth2)
- [google-apis-rs](https://github.com/Byron/google-apis-rs)

# 概要

RustからGCPの [Cloud Pub/Sub](https://cloud.google.com/pubsub/?hl=ja) にメッセージをpublishしたり、 subscribeしたりするサンプルです。  
[yup-oauth2のサンプルコード](https://github.com/dermesser/yup-oauth2/blob/master/examples/service_account/src/main.rs) を参考に・・・というかほとんどそのままです。

[yup-oauth2](https://github.com/dermesser/yup-oauth2) と [google-apis-rs](https://github.com/Byron/google-apis-rs) があれば GCPの操作系が割りと何でも簡単に出来てしまいそうです。

認証アカウントはサービスアカウントで、認証情報にはサービスアカウントの鍵(JSON)を使っています。

