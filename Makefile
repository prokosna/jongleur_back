.PHONY: keys init

keys:
	cd ./res && \
	openssl genrsa 4096 > jongleur_jwt_key_private.pem && \
	openssl rsa -in jongleur_jwt_key_private.pem -pubout -out jongleur_jwt_key_public.pem && \
	openssl rsa -in jongleur_jwt_key_private.pem -out jongleur_jwt_key_private.der -outform der && \
	openssl rsa -pubin -in jongleur_jwt_key_public.pem -out jongleur_jwt_key_public.der -outform der

init:
	docker-compose exec db mongo --eval "db.getSiblingDB('jongleur').createUser({user:'jongleur',pwd:'password',roles:[{role:'readWrite',db:'jongleur'}]});"
	docker-compose exec db mongo --eval "db.getSiblingDB('jongleur').admins.createIndex({id:1},{unique:true});"
	docker-compose exec db mongo --eval "db.getSiblingDB('jongleur').admins.createIndex({name:1},{unique:true});"
	docker-compose exec db mongo --eval "db.getSiblingDB('jongleur').access_tokens.createIndex({id:1},{unique:true});"
	docker-compose exec db mongo --eval "db.getSiblingDB('jongleur').access_tokens.createIndex({token:1},{unique:true});"
	docker-compose exec db mongo --eval "db.getSiblingDB('jongleur').clients.createIndex({id:1},{unique:true});"
	docker-compose exec db mongo --eval "db.getSiblingDB('jongleur').clients.createIndex({name:1},{unique:true});"
	docker-compose exec db mongo --eval "db.getSiblingDB('jongleur').end_users.createIndex({id:1},{unique:true});"
	docker-compose exec db mongo --eval "db.getSiblingDB('jongleur').end_users.createIndex({name:1},{unique:true});"
	docker-compose exec db mongo --eval "db.getSiblingDB('jongleur').end_users.createIndex({email:1},{unique:true});"
	docker-compose exec db mongo --eval "db.getSiblingDB('jongleur').grants.createIndex({id:1},{unique:true});"
	docker-compose exec db mongo --eval "db.getSiblingDB('jongleur').grants.createIndex({code:1},{unique:true});"
	docker-compose exec db mongo --eval "db.getSiblingDB('jongleur').id_tokens.createIndex({id:1},{unique:true});"
	docker-compose exec db mongo --eval "db.getSiblingDB('jongleur').id_tokens.createIndex({token:1},{unique:true});"
	docker-compose exec db mongo --eval "db.getSiblingDB('jongleur').refresh_tokens.createIndex({token:1},{unique:true});"
	docker-compose exec db mongo --eval "db.getSiblingDB('jongleur').resources.createIndex({id:1},{unique:true});"
	docker-compose exec db mongo --eval "db.getSiblingDB('jongleur').resources.createIndex({name:1},{unique:true});"
