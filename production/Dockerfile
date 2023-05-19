FROM debian:stable-slim
#rust:1.66


COPY Backend /usr/local/bin/
CMD ["/usr/local/bin/Backend"]

###################### For production, you use a distroless, but for debugging with distro ######################

# FROM gcr.io/distroless/cc-debian11

# COPY --from=builder /etc/passwd /etc/passwd
# COPY --from=builder /etc/group /etc/group
# COPY --from=builder /app/public /public

# COPY --from=builder /app/target/release/Backend /app/Backend
# WORKDIR /app

# USER web:web

# #CMD /app
# CMD ["./Backend"]