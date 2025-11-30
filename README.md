# Auto-Adobe

Automate your sign-in to the KMUTNB Software Center and trigger an Adobe license reservation in one shot.

This repository contains a minimal Rust script that:
- Loads your credentials from environment variables.
- Logs in to `https://software.kmutnb.ac.th/login/`.

Intended audience: students and staff who already have legitimate access and permissions granted by KMUTNB. Use responsibly and in accordance with the university policies and the website's terms of service.

> [!CAUTION]
> ## Legal and ethical notice
>
> This script is provided for personal convenience where automation is permitted. You are responsible for complying with KMUTNB policies and all applicable terms of service. Do not use this script to circumvent security, access controls, or usage limits.

## Prerequisites

- Rust and Cargo.
- A valid KMUTNB account with access to Adobe reservation.
- Network access to `software.kmutnb.ac.th`.

## Usage

- From the project root, run: `cargo run --release`

## Troubleshooting

- Login fails (401/403 or redirected back to login):
  - Verify `USERNAME` and `PASSWORD` are correct.
  - The site may require additional tokens (CSRF) or a prior GET to establish cookies. You may need to:
    - Perform a GET to the login page and parse hidden inputs.
    - Carry those tokens into the POST request.
  - The site may have introduced CAPTCHA or other bot protection, which will block automation.

## Security best practices

- Never commit `.env` or credentials. The `.gitignore` in this project already ignores `.env`.
- Rotate your password periodically and immediately if you suspect compromise.
- Use a separate, least-privilege account for automation if possible.
- When scheduling, ensure the account running the task has access to the `.env` securely and that logs do not dump secrets.
- Prefer enabling SSL verification to mitigate MITM risks.
