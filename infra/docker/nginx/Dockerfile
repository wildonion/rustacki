# Use the official Nginx image as the base image
FROM nginx:latest

WORKDIR /etc/nginx

# copy the whole project into /etc/nginx
ADD . ./ 

# go to infra/docker/nginx folder
WORKDIR /etc/nginx/infra/docker/nginx

# Remove the default Nginx configuration file
RUN rm /etc/nginx/conf.d/default.conf

# accessing logs dir requires password, copy it into the /etc/nginx dir
COPY .htpasswd /etc/nginx/.htpasswd
# Copy the SSL certificate and private key to the container
#your-app.app:
COPY cert-your-app.app.pem /etc/nginx/cert-your-app.app.pem
COPY key-your-app.app.pem /etc/nginx/key-your-app.app.pem
#api.your-app.app:
COPY cert-api.your-app.app.pem /etc/nginx/cert-api.your-app.app.pem
COPY key-api.your-app.app.pem /etc/nginx/key-api.your-app.app.pem
#event.api.your-app.app:
COPY cert-event.api.your-app.app /etc/nginx/cert-event.api.your-app.app
COPY key-event.api.your-app.app /etc/nginx/key-event.api.your-app.app
#dev.api.your-app.app:
COPY cert-dev.api.your-app.app.pem /etc/nginx/cert-dev.api.your-app.app.pem
COPY key-dev.api.your-app.app.pem /etc/nginx/key-dev.api.your-app.app.pem
#adminer.your-app.app:
COPY cert-adminer.your-app.app.pem /etc/nginx/cert-adminer.your-app.app.pem
COPY key-adminer.your-app.app.pem /etc/nginx/key-adminer.your-app.app.pem
#portainer.your-app.app:
COPY cert-your-app.app.pem /etc/nginx/cert-your-app.app.pem
COPY key-your-app.app.pem /etc/nginx/key-your-app.app.pem
#grafana.your-app.app:
COPY cert-grafana.your-app.app.pem /etc/nginx/cert-grafana.your-app.app.pem
COPY key-grafana.your-app.app.pem /etc/nginx/key-grafana.your-app.app.pem
#rmq.your-app.app:
COPY cert-rmq.your-app.app.pem /etc/nginx/cert-rmq.your-app.app.pem
COPY key-rmq.your-app.app.pem /etc/nginx/key-rmq.your-app.app.pem
#rmq.your-app.app:
COPY cert-dbeaver.your-app.app.pem /etc/nginx/cert-dbeaver.your-app.app.pem
COPY key-dbeaver.your-app.app.pem /etc/nginx/key-dbeaver.your-app.app.pem

COPY options-ssl-nginx.conf /etc/nginx/options-ssl-nginx.conf
COPY ssl-dhparams.pem /etc/nginx/ssl-dhparams.pem

# Expose port 80 for HTTP traffic and port 443 for HTTPS traffic
EXPOSE 80
EXPOSE 443

COPY nginx.conf /etc/nginx/nginx.conf
COPY your-app.app.conf /etc/nginx/your-app.app.conf
COPY api.your-app.app.conf /etc/nginx/api.your-app.app.conf
COPY event.api.your-app.app.conf /etc/nginx/event.api.your-app.app.conf
COPY adminer.your-app.app.conf /etc/nginx/adminer.your-app.app.conf
COPY grafana.your-app.app.conf /etc/nginx/grafana.your-app.app.conf
COPY rmq.your-app.app.conf /etc/nginx/rmq.your-app.app.conf
COPY dbeaver.your-app.app.conf /etc/nginx/dbeaver.your-app.app.conf

# Start Nginx in the foreground when the Docker container starts
CMD ["nginx", "-g", "daemon off;"]