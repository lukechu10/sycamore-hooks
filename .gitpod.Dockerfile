FROM gitpod/workspace-rust

# Install wasm-pack
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Install Rust with wasm32-unknown-unknown target on nightly toolchain
# We unset CARGO_HOME to prevent errors with rustup
ENV CARGO_HOME=
RUN rustup toolchain add nightly
RUN rustup default nightly
RUN rustup target add wasm32-unknown-unknown
