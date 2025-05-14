# define builder image ------------------------------------------------------------------
FROM alpine:3.21.3 AS builder

WORKDIR /kojamp_app
RUN apk update
RUN apk add build-base git curl
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

COPY Cargo.* .
COPY src ./src

RUN cargo build --release

# define a separated runner image -------------------------------------------------------
FROM alpine:3.21.3 AS runner
RUN apk upgrade && apk add git openjdk17 bash

# Create user and set up home directory
RUN adduser -D -s /bin/bash -h /home/kojampuser kojampuser && \
    mkdir -p /home/kojampuser/projects;


COPY scripts/.bash_profile /tmp/.bash_profile
RUN sed -i 's/\r$//' /tmp/.bash_profile && \
    mv /tmp/.bash_profile /home/kojampuser/.bash_profile && \
    chown kojampuser:kojampuser /home/kojampuser/.bash_profile

COPY scripts/.bashrc /tmp/.bashrc
RUN sed -i 's/\r$//' /tmp/.bashrc && \
    mv /tmp/.bashrc /home/kojampuser/.bashrc && \
    chown kojampuser:kojampuser /home/kojampuser/.bashrc

RUN chown -R kojampuser:kojampuser /home/kojampuser

USER kojampuser
WORKDIR /home/kojampuser
COPY --chown=kojampuser:kojampuser --from=builder /kojamp_app/target/release/kojamp /bin/

# Default command (optional)
CMD ["bash", "-l"]
