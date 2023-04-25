# StatEye :eye_speech_bubble:

An eye that keeps track of your Roblox status and shares it with others.

![application](res/roblox-presence.png) ![application](res/roblox-studio-presence.png)

Roblox presence for Discord with only one native standalone executable that relies on zero external dependencies, and doesn't need to be installed.

## Performance :zap:

| Executable Size | Memory Usage     | Network Usage     | Disk Usage | CPU Usage |
|-----------------|------------------|-------------------|------------|-----------|
| ~1.5MB          | ~1.5MB to ~2.1MB | 0MB/s to ~0.1MB/s | 0%         | 0%        |

*0% does **NOT** imply that the program is not utilizing the given resource, but rather indicate that the usage is too low to be represented by one decimal place percentage.*

## Installation :building_construction:

Just download the `stateye.exe` file from the latest release, and run it.

## Configuration File :gear:

Configuration file is optional, if you choose to include it, do the following:

- Create a file called `stateye.config` and place it in the same dictionary where `stateye.exe` is located.
- Open `stateye.config` in a notepad or other text editing program and paste the following:

```txt
token=YOUR_ROBLOX_SECURITY_TOKEN (optional)
website=true (optional)
player=true (optional)
studio=true (optional)
```

Format: `key=value`.

|   Key   | Description                                                                  | Value Type | Default Value |
|:-------:|------------------------------------------------------------------------------|:----------:|:-------------:|
|  token  | Roblox account token which can be found in a cookie called `.ROBLOSECURITY`. |  `string`  |               |
| website | Whether to show that you are online.                                         |   `bool`   |     `true`    |
|  player | Whether to show that you are playing a game.                                 |   `bool`   |     `true`    |
|  studio | Whether to show that you are developing a game.                              |   `bool`   |     `true`    |

> :warning: Do **NOT** share the token with anyone, keep it safe as it will allow bad actor to bypass all security features and access your account!

## Reporting Bugs :bug:

1. Download `stateye_debug.exe` from the latest release.
2. Start screen recording.
3. Do something to cause problems and then exit out of the program if it didn't already.
4. Stop screen recording.
5. Remove sensitive details from the console (if any) and the screen recording.
6. Open new issue and include the console logs with a screen recording along with details, such as what operating system you're using, etc.
