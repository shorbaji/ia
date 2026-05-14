# Authentication

`ia` uses a token stored at `~/.config/insaali/credentials` (mode `0600`). The token is obtained by signing in through your browser.

## Sign in

```sh
ia login
```

Opens your browser to sign in with GitHub or Google, then writes the token to `~/.config/insaali/credentials`. The browser tab confirms when you can close it.

## Sign out

```sh
ia logout
```

Removes the saved token. If no token is present, prints `not signed in.`
