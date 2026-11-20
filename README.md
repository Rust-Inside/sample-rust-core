# Sample Rust Core

[English](README.md) | [한국어](README.ko.md)

Install the `v0.2.0` SDK for your platform to try the sample Rust library. In an app, call `account.getMe()` to fetch the signed-in user's profile. On a JVM server, use the SDK to build the profile response. [Rust source for v0.2.0](https://github.com/Rust-Inside/sample-rust-core/blob/v0.2.0/src/api.rs)

[iOS](#ios) · [Android](#android) · [Flutter](#flutter) · [Web](#web) · [JVM server](#jvm-server)

To run the client examples, point `apiOrigin` at your account server. The Rust client sends `GET /v1/me` with the `authorization` value you supply (for example, `Bearer <token>`). The server must return JSON with `id`, `handle`, and `displayName`.

## iOS

In Xcode, open **File → Add Package Dependencies**, enter the repository URL, and select **Exact Version** `0.2.0`. Add the `SampleRustCoreSDK` product to your app target. The Swift package supports iOS 18 or later on arm64 devices and simulators. Xcode downloads the prebuilt XCFramework for the selected version. [iOS installation guide](https://github.com/Rust-Inside/sample-ios-sdk)

```text
https://github.com/Rust-Inside/sample-ios-sdk.git
```

```swift
import SampleRustCoreSDK

func displayName(apiOrigin: String, authorization: String) async throws -> String {
    let account = AccountClient(baseUrl: apiOrigin, authorization: authorization)
    let me = try await account.getMe()
    return me.displayName
}
```

## Android

The Android SDK supports API level 23 and later. Register the release URL as an Ivy repository in Gradle so it can fetch the AAR and its metadata. Add the dependency to your app module's `build.gradle.kts` and declare `android.permission.INTERNET` in the app manifest. [Android installation guide](https://github.com/Rust-Inside/sample-android-sdk)

```kotlin
dependencies {
    implementation("dev.rustinside.sample_rust_coresdk:sample-rust-core-android:0.2.0")
}
```

```kotlin
import dev.rustinside.sample_rust_coresdk.AccountClient

suspend fun displayName(apiOrigin: String, authorization: String): String =
    AccountClient(apiOrigin, authorization).use { account ->
        account.getMe().displayName
    }
```

## Flutter

Flutter apps use the same Dart package on iOS and Android. Add the dependency to `pubspec.yaml` and run `flutter pub get`. The `path: flutter` entry points to the package inside the SDK repository. [Flutter installation guide](https://github.com/Rust-Inside/sample-flutter-sdk)

```yaml
dependencies:
  sample_rust_core_flutter:
    git:
      url: https://github.com/Rust-Inside/sample-flutter-sdk.git
      ref: v0.2.0
      path: flutter
```

```dart
import 'package:sample_rust_core_flutter/sample_rust_core_flutter.dart';

Future<String> displayName(String apiOrigin, String authorization) async {
  final account = AccountClient(baseUrl: apiOrigin, authorization: authorization);
  try {
    final me = await account.getMe();
    return me.displayName;
  } finally {
    account.close();
  }
}
```

## Web

Install the release archive with npm from your web project. [Web installation guide](https://github.com/Rust-Inside/sample-web-sdk)

```sh
npm install https://github.com/Rust-Inside/sample-web-sdk/releases/download/v0.2.0/sample-rust-core-web-0.2.0.tgz
```

Call `init()` once at startup to load WebAssembly. Configure your bundler to ship the package's `index_bg.wasm` file with your app.

```js
import init, { AccountClient } from 'sample-rust-core-web';

await init();

export async function displayName(apiOrigin, authorization) {
  const account = new AccountClient(apiOrigin, authorization);
  let me;
  try {
    me = await account.getMe();
    return me.displayName;
  } finally {
    me?.free();
    account.free();
  }
}
```

## JVM server

The Kotlin/Spring server uses the JVM SDK as a JAR and requires JVM 17 or later. Register the release URL as an Ivy repository in Gradle and add this dependency to the server module's `build.gradle.kts`. Gradle downloads the JAR and its metadata. [JVM installation guide](https://github.com/Rust-Inside/sample-server-sdk)

```kotlin
dependencies {
    implementation("dev.rustinside.sample_rust_coresdk:sample-rust-core-jvm:0.2.0")
}
```

Declare `GET /v1/me` in your Spring controller and configure Spring Security to require authentication for that route. In this example, your application provides `AccountService.requireByUsername` to look up the authenticated account. The controller sends the JSON returned by `meResponseJson()`.

```kotlin
import dev.rustinside.sample_rust_coresdk.meResponseJson
import java.security.Principal
import org.springframework.http.MediaType
import org.springframework.web.bind.annotation.GetMapping
import org.springframework.web.bind.annotation.RestController

@RestController
class ProfileController(private val accounts: AccountService) {
    @GetMapping("/v1/me", produces = [MediaType.APPLICATION_JSON_VALUE])
    fun getMe(principal: Principal): String {
        val account = accounts.requireByUsername(principal.name)
        return meResponseJson(account.id, account.handle, account.nickname)
    }
}
```

As of `v0.2.0`, the client keeps its request path in a private Rust constant. If upgrading from `v0.1.0`, declare `/v1/me` in Spring in place of `mePath()`. [Rust source](src/api.rs)

The Rust module marked `#[rust_inside::server]` exposes `meResponseJson()` to the server SDK only. A server that calls another account service can also use `AccountClient.getMe()`.

## Local tests

Run the tests from the library root with stable Rust. The integration test starts a local HTTP server.

```sh
cargo test --locked
```

## Release workflow reference

The manifest lists the Rust API module and the five SDK repositories. Its `requiredApi` entries check `AccountClient.getMe()` in the app and web SDKs and `meResponseJson()` in the JVM SDK. The sample workflow runs manually from **Actions → Release sample SDKs → Run workflow**. [Manifest](rust-inside.json) · [Workflow](.github/workflows/release.yml)

Keep `dry_run` enabled to build without publishing. Enter an existing tag such as `v0.2.0` in `release_tag` to build that exact version, or leave it blank to try the selected branch as `0.0.0`. To publish from Actions, enter a tag and disable `dry_run`. That requires the `RUST_INSIDE_RELEASE_TOKEN` secret, with **Contents: Read and write** on the destination SDK repositories.

## License

[MIT](LICENSE)
