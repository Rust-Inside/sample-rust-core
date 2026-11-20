# Sample Rust Core

[English](README.md) | [한국어](README.ko.md)

Install the `v0.1.0` SDK for your platform to try the sample Rust library. In an app, call `account.getMe()` to fetch the signed-in user's profile. On a JVM server, use the SDK to build the profile response. [Rust source for v0.1.0](https://github.com/Rust-Inside/sample-rust-core/blob/v0.1.0/src/api.rs)

[iOS](#ios) · [Android](#android) · [Flutter](#flutter) · [Web](#web) · [JVM server](#jvm-server)

To run the client examples, point `apiOrigin` at your account server. The Rust client sends `GET /v1/me` with the `authorization` value you supply (for example, `Bearer <token>`). The server must return JSON with `id`, `handle`, and `displayName`.

## iOS

In Xcode, open **File → Add Package Dependencies**, enter the repository URL, and select **Exact Version** `0.1.0`. Add the `SampleRustCoreSDK` product to your app target. The Swift package supports iOS 18 or later on arm64 devices and simulators. Xcode downloads the prebuilt XCFramework for the selected version. [iOS installation guide](https://github.com/Rust-Inside/sample-ios-sdk)

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
    implementation("dev.rustinside.sample_rust_coresdk:sample-rust-core-android:0.1.0")
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
      ref: v0.1.0
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
npm install https://github.com/Rust-Inside/sample-web-sdk/releases/download/v0.1.0/sample-rust-core-web-0.1.0.tgz
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
    implementation("dev.rustinside.sample_rust_coresdk:sample-rust-core-jvm:0.1.0")
}
```

`mePath()` returns `/v1/me`. The server authenticates requests at that route and looks up the account. Pass the account fields to `meResponseJson` to build the JSON response that the app's `getMe()` reads.

```kotlin
import dev.rustinside.sample_rust_coresdk.meResponseJson
import org.springframework.http.MediaType
import org.springframework.http.ResponseEntity

fun profileResponse(id: String, handle: String, nickname: String): ResponseEntity<String> =
    ResponseEntity.ok()
        .contentType(MediaType.APPLICATION_JSON)
        .body(meResponseJson(id, handle, nickname))
```

The Rust module marked `#[rust_inside::server]` exposes `mePath()` and `meResponseJson()` to the server SDK only. A server that calls another account service can also use `AccountClient.getMe()`.

## Local tests

Run the tests from the library root with stable Rust. The integration test starts a local HTTP server.

```sh
cargo test --locked
```

## Release workflow reference

The manifest lists the Rust API module and the five SDK repositories. The sample workflow runs manually from **Actions → Release sample SDKs → Run workflow**. [Manifest](rust-inside.json) · [Workflow](.github/workflows/release.yml)

Leave `release_tag` blank for a dry run that builds and packages version `0.0.0`. To publish from an existing core tag, enter that tag in `release_tag`. Only publishing requires the `RUST_INSIDE_RELEASE_TOKEN` secret, with **Contents: Read and write** on the destination SDK repositories.

## License

[MIT](LICENSE)
