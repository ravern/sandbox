#! /bin/bash

# Fix locale warning.
locale-gen UTF-8 > /dev/null

# Find and kill the infinite loop process.
pid=$(pgrep infinite)
kill $pid > /dev/null

# Generate the self-signed certificate.
openssl req -x509 -nodes -days 365 -newkey rsa:2048 -keyout /etc/ssl/private/selfsigned.key -out /etc/ssl/certs/selfsigned.crt -subj "/C=SG/ST=Singapore/L=Singapore/O=Ravern Koh/OU=Solo/CN=localhost"

# Disable the default configuration.
a2dissite 000-default.conf > /dev/null

# Copy in the configuration and enable it.
cp /tmp/site.conf /etc/apache2/sites-available/site.conf > /dev/null
a2ensite site.conf > /dev/null

# Enable the `ssl` module on Apache.
a2enmod ssl > /dev/null

# Install memcached.
apt-get update > /dev/null
apt-get install memcached > /dev/null

# Install Python memcached client.
pip install pymemcache > /dev/null

# Setup the cronjob.
crontab /tmp/crontab > /dev/null

# Setup the Flask app.
cp /tmp/app.py /var/www/app/app.py
cp /tmp/app.html.j2 /var/www/app/templates/app.html.j2

# Restart Apache.
service apache2 restart
