# metacat
[![My Skills](https://skillicons.dev/icons?i=rust,discord,bots)](https://skillicons.dev)

[![Codacy Badge](https://app.codacy.com/project/badge/Grade/422c1f2b2d5b4638911cacb28165503d)](https://www.codacy.com/gh/ScrexyScroo/metacat/dashboard?utm_source=github.com&amp;utm_medium=referral&amp;utm_content=ScrexyScroo/metacat&amp;utm_campaign=Badge_Grade)

> **Warning** <br />
> This code is hot garbage

> **Special thanks** to Intellisense for helping me glue this together

![metacat-2023-01-22-1225(1)](https://user-images.githubusercontent.com/30901276/213905363-cc05ec10-ee07-478b-8985-b1b3960cddfc.png)

## Steps to setting it up
- Put your bot's token into `discordtoken.txt`, place it in the project root or with the binary you have. If you haven't already setup a `bot account` go to the [discord dev portal](https://discord.com/developers/applications) and create an app
- Download `clientsecret.json` from https://console.cloud.google.com/apis/credentials for the project you created and put it in the project root or the app directory

> **Note** <br />
> You would need to accept *rclone access request* via a browser on the initial run, however it would automatically cache the recieved token into `tokencache.json` so this is a one-time step until the token expires
