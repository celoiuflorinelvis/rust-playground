FROM rust:latest

ENV DEBIAN_FRONTEND=noninteractive

#RUN cargo install --path .

# Install needed packages and setup non-root user
RUN apt-get update \
    && apt-get -y install --no-install-recommends apt-utils 2>&1 \
    #
    # Verify git, process tools, lsb-release (common in install instructions for CLIs) installed
    && apt-get -y install git procps lsb-release \
    #
    # Create a non-root user to use if preferred - see https://aka.ms/vscode-remote/containers/non-root-user.
    && groupadd --gid 1000 developer \
    && useradd --uid 1000 --gid 1000 --shell /bin/bash --create-home developer \
    # [Optional] Add sudo support for the non-root user
    && apt-get install -y sudo \
    && echo "developer ALL=(ALL) NOPASSWD:ALL" > /etc/sudoers.d/developer \
    #
    # Clean up
    && apt-get autoremove -
