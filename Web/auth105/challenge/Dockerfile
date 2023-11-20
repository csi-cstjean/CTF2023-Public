FROM nginx:1.25

WORKDIR /
COPY ./html2/ /usr/share/nginx/html
COPY ./config/nginx/conf.d/default.conf /etc/nginx/conf.d/default.conf
COPY ./config/nginx/nginx.conf /etc/nginx/nginx.conf
RUN chown -R nginx:nginx /usr/share/nginx/html
RUN chmod -R 755 /usr/share/nginx/html

EXPOSE 80

RUN mv /usr/share/nginx/html/git /usr/share/nginx/html/.git
