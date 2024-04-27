#!/bin/bash


echo "-> wanna add (sub)domain? "
read is_new_domain
cd ..

if [[ $is_new_domain == "Y" || $is_new_domain == "y" ]]; then
    echo "-> enter domain? "
    read DOMAIN
    echo "creating new SSL certificate and key files for $DOMAIN using certbot,
    ensure that you have a (sub)domain points to this machine and it can accepts inbound connections 
    from the internet also make sure that necessary ports like 80 and 443 are opened"
    NGINX_CONTAINER_ID=$(docker container ls  | grep 'nginx' | awk '{print $1}')
    # stop nginx docker cause certbox needs the port 80 and 443
    sudo docker stop $NGINX_CONTAINER_ID && sudo certbot certonly --standalone -d $DOMAIN && sudo docker start $NGINX_CONTAINER_ID
    sudo cp /etc/letsencrypt/live/$DOMAIN/fullchain.pem $(pwd)/infra/cert/cert-$DOMAIN.pem && sudo cp /etc/letsencrypt/live/$DOMAIN/fullchain.pem $(pwd)/infra/docker/nginx/cert-$DOMAIN.pem
    sudo cp /etc/letsencrypt/live/$DOMAIN/privkey.pem $(pwd)/infra/cert/key-$DOMAIN.pem && sudo cp /etc/letsencrypt/live/$DOMAIN/privkey.pem $(pwd)/infra/docker/nginx/key-$DOMAIN.pem
    echo "okay now you can use $(pwd)/infra/docker/nginx/key-$DOMAIN.pem and $(pwd)/infra/docker/nginx/cert-$DOMAIN.pem in your nginx Dockerfile and $DOMAIN.conf"
else
    echo "🤔"
fi

# eventually redeploy nginx anyway cause we might have added new cert and config files
echo "[🛰] redeploying nginx docker container"
docker system prune --all
sudo docker stop nginx
sudo docker rm -f nginx
sudo docker build -t --no-cache nginx -f $(pwd)/infra/docker/nginx/Dockerfile .
sudo docker run -d -it -p 80:80 -p 443:443 -v $(pwd)/infra/data/nginx/confs/:/etc/nginx -v $(pwd)/infra/data/nginx/wwws/:/usr/share/nginx/ -v $(pwd)/assets/:/etc/nginx/assets -v $(pwd)/infra/logs/:/etc/nginx/logs --name nginx --network host nginx