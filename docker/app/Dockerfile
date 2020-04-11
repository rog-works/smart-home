FROM rust:1-slim-buster

# install openssh-server
RUN apt-get update \
 && apt-get install -y openssl openssh-server \
 && echo 'Port 22222' >> /etc/ssh/sshd_config \
 && echo 'PermitRootLogin no' >> /etc/ssh/sshd_config \
 && echo 'PasswordAuthentication no' >> /etc/ssh/sshd_config

# add user
RUN echo 'auth	required	pam_wheel.so' >> /etc/pam.d/su \
 && addgroup --gid 11 wheel \
 && apt install sudo \
 && echo '%wheel	ALL=(ALL)	ALL' > /etc/sudoers.d/sudoers \
 && echo '%wheel	ALL=(ALL)	NOPASSWD: ALL' >> /etc/sudoers.d/sudoers \
 && useradd -G wheel -m worker -s /bin/bash

COPY ./.ssh/authorized_keys /home/worker/.ssh/authorized_keys
COPY ./entrypoint.sh /home/worker/entrypoint.sh

RUN chown worker:worker /home/worker/entrypoint.sh \
 && chmod 700 /home/worker/entrypoint.sh \
 && chmod 700 /home/worker/.ssh \
 && chmod 600 /home/worker/.ssh/authorized_keys \
 && chown worker:worker /home/worker/.ssh \
 && chown worker:worker /home/worker/.ssh/authorized_keys

RUN apt-get install -y curl git

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

USER worker
ENV USER=worker

WORKDIR /opt/app

CMD /home/worker/entrypoint.sh
