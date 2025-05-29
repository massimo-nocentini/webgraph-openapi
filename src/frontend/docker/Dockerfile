
FROM nginx:stable-alpine


COPY docker/server.nginx.conf /etc/nginx/conf.d/default.conf
COPY html /usr/share/nginx/html

EXPOSE 80

# CMD ["nginx", "-g", "daemon off;"]