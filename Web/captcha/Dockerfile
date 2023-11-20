FROM php:8.2-apache

RUN apt-get update --fix-missing
RUN apt-get install -y build-essential libjpeg-dev libfreetype6-dev

RUN docker-php-ext-install mysqli

RUN docker-php-ext-configure gd --with-freetype=/usr/include/ --with-jpeg=/usr/include/ \
    && docker-php-ext-install gd