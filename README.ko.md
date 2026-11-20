# Sample Rust Core

[English](README.md)

플랫폼에 맞는 `v0.2.0` SDK를 설치해 샘플 Rust 라이브러리를 사용해 보세요. 앱에서는 `account.getMe()`로 로그인한 사용자의 프로필을 조회하고, JVM 서버에서는 SDK로 프로필 응답을 만듭니다. [v0.2.0 Rust 소스](https://github.com/Rust-Inside/sample-rust-core/blob/v0.2.0/src/api.rs)

[iOS](#ios) · [Android](#android) · [Flutter](#flutter) · [웹](#웹) · [JVM 서버](#jvm-서버)

클라이언트 예제를 실행할 때는 `apiOrigin`에 계정 서버 주소를 넣으세요. Rust 클라이언트는 `Bearer <token>`처럼 `authorization`에 전달한 값을 헤더에 담아 `GET /v1/me`를 요청합니다. 서버는 `id`, `handle`, `displayName`이 담긴 JSON을 반환해야 합니다.

## iOS

Xcode의 **File → Add Package Dependencies**에서 저장소 URL을 입력하고 **Exact Version**에서 `0.2.0`을 선택한 뒤 앱 타깃에 `SampleRustCoreSDK`를 추가하세요. Swift 패키지는 iOS 18 이상의 arm64 기기와 시뮬레이터를 지원합니다. Xcode가 선택한 버전의 XCFramework를 내려받습니다. [iOS 설치 가이드](https://github.com/Rust-Inside/sample-ios-sdk/blob/main/README.ko.md)

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

Android SDK는 API 23 이상에서 쓸 수 있습니다. Gradle에 릴리스 URL을 Ivy 저장소로 등록하면 AAR과 메타데이터를 자동으로 내려받습니다. 앱 모듈의 `build.gradle.kts`에 아래 의존성을 추가하고 앱 매니페스트에 `android.permission.INTERNET` 권한을 선언합니다. [Android 설치 가이드](https://github.com/Rust-Inside/sample-android-sdk/blob/main/README.ko.md)

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

Flutter 앱은 iOS와 Android에서 같은 Dart 패키지를 씁니다. `pubspec.yaml`에 의존성을 추가하고 `flutter pub get`을 실행하세요. `path: flutter`는 SDK 저장소에서 패키지가 있는 디렉터리입니다. [Flutter 설치 가이드](https://github.com/Rust-Inside/sample-flutter-sdk/blob/main/README.ko.md)

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

## 웹

웹 프로젝트에서 릴리스 패키지를 npm으로 설치하세요. [웹 설치 가이드](https://github.com/Rust-Inside/sample-web-sdk/blob/main/README.ko.md)

```sh
npm install https://github.com/Rust-Inside/sample-web-sdk/releases/download/v0.2.0/sample-rust-core-web-0.2.0.tgz
```

앱 시작 시 `init()`을 한 번 호출해 WebAssembly를 초기화합니다. 번들러가 패키지의 `index_bg.wasm` 파일도 앱과 함께 배포하도록 설정하세요.

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

## JVM 서버

Kotlin/Spring 서버는 JVM 17 이상에서 서버 SDK를 JAR로 받아 씁니다. Gradle에 릴리스 URL을 Ivy 저장소로 등록한 뒤 서버 모듈의 `build.gradle.kts`에 의존성을 추가하세요. Gradle이 JAR과 메타데이터를 내려받습니다. [JVM 설치 가이드](https://github.com/Rust-Inside/sample-server-sdk/blob/main/README.ko.md)

```kotlin
dependencies {
    implementation("dev.rustinside.sample_rust_coresdk:sample-rust-core-jvm:0.2.0")
}
```

Spring 컨트롤러에 `GET /v1/me`를 선언하고, Spring Security에서 이 경로에 인증을 요구하도록 설정하세요. 예제의 `AccountService.requireByUsername`은 앱에서 구현하는 계정 조회 메서드입니다. 컨트롤러는 인증된 계정의 데이터를 `meResponseJson()`에 넘겨 JSON 응답을 보냅니다.

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

`v0.2.0`부터 클라이언트의 요청 경로는 Rust 내부 상수로 관리합니다. `v0.1.0`에서 올릴 때는 Spring에서 `mePath()`를 호출하던 자리에 `/v1/me`를 선언하세요. [Rust 소스](src/api.rs)

`meResponseJson()`은 Rust에서 `#[rust_inside::server]`를 붙인 모듈에 있어 서버 SDK에서 호출할 수 있습니다. 다른 계정 서버에 요청을 보낼 때는 서버에서도 `AccountClient.getMe()`를 쓸 수 있습니다.

## 로컬 테스트

Rust stable을 설치하고 라이브러리 루트에서 테스트를 실행하세요. 통합 테스트는 로컬 HTTP 서버를 띄웁니다.

```sh
cargo test --locked
```

## 릴리스 워크플로 참고

설정 파일에는 공개할 Rust API 모듈과 SDK를 받을 저장소 다섯 곳이 지정돼 있습니다. `requiredApi`로 앱과 웹에서 `AccountClient.getMe()`, JVM에서 `meResponseJson()`을 검사합니다. 샘플 워크플로는 **Actions → Release sample SDKs → Run workflow**에서 수동으로 실행합니다. [배포 설정](rust-inside.json) · [워크플로](.github/workflows/release.yml)

게시 없이 빌드만 하려면 `dry_run`을 켜 두세요. `release_tag`에 `v0.2.0` 같은 태그를 입력하면 그 버전을 빌드하고, 비워 두면 선택한 브랜치를 `0.0.0`으로 빌드합니다. Actions에서 게시하려면 태그를 입력하고 `dry_run`을 끄세요. 이때는 대상 SDK 저장소에 **Contents: Read and write** 권한이 있는 `RUST_INSIDE_RELEASE_TOKEN` 시크릿이 필요합니다.

## 라이선스

[MIT](LICENSE)
