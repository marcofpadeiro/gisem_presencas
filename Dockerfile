FROM rust:latest

# Install Firefox
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    firefox-esr \
    wget \
    libgtk-3-0 \
    libdbus-glib-1-2 \
    cron \ 
    bzip2 \
    && rm -rf /var/lib/apt/lists/*


# Install Geckodriver
RUN GECKODRIVER_VERSION=`wget -qO- "https://api.github.com/repos/mozilla/geckodriver/releases/latest" | grep -Po '"tag_name": "\K.*?(?=")'` \
    && wget --no-verbose -O /tmp/geckodriver.tar.gz "https://github.com/mozilla/geckodriver/releases/download/$GECKODRIVER_VERSION/geckodriver-$GECKODRIVER_VERSION-linux64.tar.gz" \
    && tar -zxf /tmp/geckodriver.tar.gz -C /usr/local/bin \
    && rm /tmp/geckodriver.tar.gz

WORKDIR /usr/src/app
COPY . .

RUN chmod +x /usr/src/app/scripts/runner.sh
RUN chmod +x /usr/src/app/scripts/entrypoint.sh

RUN cargo build --release

# min hour day month day_of_week command
RUN echo "30 18 * * 1,2,4 /usr/src/app/scipts/runner.sh 120 300" >> mycron \
    && echo "0 21 * * 1,2,5 /usr/src/app/scripts/runner.sh 180 300" >> mycron \
    && echo "0 23 * * 4 /usr/src/app/scripts/runner.sh 180 30" >> mycron \
    && crontab mycron && rm mycron

ENTRYPOINT ["./scripts/entrypoint.sh"]
