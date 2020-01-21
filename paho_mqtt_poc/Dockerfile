FROM rustembedded/cross:mipsel-unknown-linux-musl-0.1.15
ENV OPENSSL_SEARCH_PATH "/openssl"
ENV OPENSSLCRYPTO_LIB "/openssl/lib"
ENV OPENSSL_LIB "/openssl/lib"
ENV OPENSSL_INCLUDE "/openssl/include"
ENV LD_LIBRARY_PATH "/openssl/lib"
RUN ln -s /openssl/lib /openssl/lib64
RUN ln -s /openssl/lib/libssl.a /usr/local/mipsel-linux-muslsf/lib/libssl.a
RUN ln -s /openssl/lib/libcrypto.a /usr/local/mipsel-linux-muslsf/lib/libcrypto.a