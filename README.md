# Jongleur Back

This is the backend app of [Jongleur](https://github.com/prokosna/jongleur).

## Prerequisite

### Keys

- Generate RSA Keys in the 'res' directory.

```
$ openssl genrsa 4096 > jongleur_jwt_key_private.pem
$ openssl rsa -in jongleur_jwt_key_private.pem -pubout -out jongleur_jwt_key_public.pem
$ openssl rsa -in jongleur_jwt_key_private.pem -out jongleur_jwt_key_private.der -outform der
$ openssl rsa -pubin -in jongleur_jwt_key_public.pem -out jongleur_jwt_key_pub.der -outform der
```

- Rename key files according to the 'Config.yml'.

```
jwt_private_key: ./res/jongleur_jwt_key_private.der
jwt_public_key: ./res/jongleur_jwt_key_pub.der
jwt_public_key_pem: ./res/jongleur_jwt_key_pub.pem
```