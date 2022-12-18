# Steps

This covers some of the steps I took while investigating. It doesn't fully represent the steps within the provisioning script.

_Note: In the submitted VM the locale isn't fixed, as I just couldn't get it to work within the provisioning script. Also, the redirect goes to port 443, which is not the correct port due to port fowarding. Thus, the HTTP to HTTPS redirection doesn't work unless the port forwarding is changed to forward 80 to 80 and 443 to 443._

## Basics of Vagrant

Bringing the VM up and running.

```bash
$ vagrant up
```

SSH into the VM.

```bash
$ vagrant ssh
```

## Analyzing system health

### New release available

I don't think I should change this without understanding more about the project first.

### Invalid locale warning

Immediately after SSH-ing into the box, there's a warning for invalid locale.

```bash
WARNING! Your environment specifies an invalid locale.
 This can affect your user experience significantly, including the
 ability to manage packages. You may install the locales by running:

   sudo apt-get install language-pack-UTF-8
     or
   sudo locale-gen UTF-8
```

I simply ran the following command to fix it.

```bash
$ sudo locale-gen UTF-8
```

### Some infinite loop

I ran `top` to check on the running processes and found that there's some kind of infinite loop taking up 97% of the CPU. No wonder my fan was running. To fix this, I simply kill the process with the PID displayed `top`.

```bash
$ pid=$(pgrep infinite)
$ sudo kill $pid
```

## (Re)starting Apache


### Checking the status of Apache

I used the `apache2ctl` command to find out that Apache isn't running.

```bash
$ apache2ctl status
 * apache2 is not running
```

### Trying to restart Apache

Tried to restart Apache, however there was an error with the config.

```bash
$ apache2ctl start
 * Starting web server apache2                                            *
 * The apache2 configtest failed.
Output of config test was:
apache2: Syntax error on line 219 of /etc/apache2/apache2.conf: Syntax error on line 3 of /etc/apache2/sites-enabled/site.conf: </VirtualHost> directive missing closing '>'
Action 'configtest' failed.
The Apache error log may have more information.
```

### Fixing the configuration

I simply fixed the error in the configuration as shown in the diff below.

```
- </VirtualHost
+ </VirtualHost>
```

### Starting and testing Apache

Finally, I managed to start Apache.

```bash
$ sudo apache2ctl start
```

In order to ensure that it was working, I simply visited `http://localhost:8080/app`.

## Configuring HTTPS on Apache

### Generate the certificate

First, the self-signed certificate needs to be generated.

```bash
$ sudo openssl req -x509 -nodes -days 365 -newkey rsa:2048 -keyout /etc/ssl/private/selfsigned.key -out /etc/ssl/certs/selfsigned.crt
```

### Enable the SSL module

The SSL module also needs to be enabled.

```bash
$ sudo a2enmod ssl
```

### Configure the site to use HTTPS

Next, the Apache configuration should be edited to include HTTPS. This was done by simply editing `/etc/apache2/sites-enabled/000-default.conf`. I moved the existing configuration from the VirtualHost on `*:80` to a VirtualHost on `*:443`, and configured the `*:80` one to redirect to HTTPS.

```
Redirect permanent / https://%{HTTP_HOST}/
```

## Install Memcached

### Install package

Can simply use `apt-get` to install it.

```bash
$ sudo apt-get update
$ sudo apt-get install memcached
```

### Securing the configuration

I ensured that memcached was listening on the local interface, by checking that the following line was within the configuration.

```
-l 127.0.0.1
```

### Install Python client

In order to use it within the Flask app, the `pymemcache` package also needs to be installed.

```bash
$ sudo pip install pymemcache
```

## Setup cronjob

Next thing to do is to setup the custom cronjob to run `exercise-memcached.sh` once every minute, by adding the following line into `crontab -e`.

```
* * * * * /home/vagrant/exercise-memcached.sh
```

## Write Python application

The completed Python application can be found in `files/app.py` and `files/app.html.j2`.
