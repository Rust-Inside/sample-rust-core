# Sample Rust Core

[English](README.md) | [한국어](README.ko.md)

로그인한 사용자의 프로필을 조회하는 Rust 라이브러리입니다. 코어에 버전 태그를 푸시하면 Rust Inside가 iOS·Android·Flutter·웹·JVM 서버용 SDK를 빌드하고 배포합니다. 앱은 자기 플랫폼용 패키지를 설치한 뒤 `account.getMe()`를 호출합니다. Rust 클라이언트가 세션의 인증 헤더를 담아 `GET /v1/me`를 요청하고, `id`, `handle`, `displayName`이 담긴 `Me`를 반환합니다. [Rust 구현](src/api.rs)

[iOS](#ios) · [Android](#android) · [Flutter](#flutter) · [웹](#웹) · [JVM 서버](#jvm-서버) · [SDK 배포](#sdk-배포)

아래 예제는 SDK `0.1.0`을 설치합니다. `getMe()`를 호출할 때는 `GET /v1/me` 요청에 위 프로필 필드를 반환하는 계정 서버가 필요합니다. `apiOrigin`에는 그 서버의 주소를, `authorization`에는 세션의 인증 헤더 값을 넣으세요. Bearer 인증을 쓰면 `Bearer <token>` 전체를 전달합니다.

## iOS

Xcode의 **File → Add Package Dependencies**에서 저장소 URL과 버전을 선택하고 앱 타깃에 `SampleRustCoreSDK`를 추가하세요. Swift 패키지는 iOS 18 이상의 arm64 기기와 시뮬레이터를 지원합니다. Xcode가 선택한 버전의 XCFramework를 내려받습니다. [iOS 설치 가이드](https://github.com/Rust-Inside/sample-ios-sdk/blob/main/README.ko.md)

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

Flutter 앱은 iOS와 Android에서 같은 Dart 패키지를 씁니다. `pubspec.yaml`에 의존성을 추가하고 `flutter pub get`을 실행하세요. `path: flutter`는 SDK 저장소에서 패키지가 있는 디렉터리입니다. [Flutter 설치 가이드](https://github.com/Rust-Inside/sample-flutter-sdk/blob/main/README.ko.md)

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

## 웹

웹 프로젝트에서 릴리스 패키지를 npm으로 설치하세요. [웹 설치 가이드](https://github.com/Rust-Inside/sample-web-sdk/blob/main/README.ko.md)

```sh
npm install https://github.com/Rust-Inside/sample-web-sdk/releases/download/v0.1.0/sample-rust-core-web-0.1.0.tgz
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
    implementation("dev.rustinside.sample_rust_coresdk:sample-rust-core-jvm:0.1.0")
}
```

`mePath()`는 `/v1/me`를 반환합니다. 서버는 이 경로에서 요청을 인증하고 계정을 조회합니다. 계정 정보를 `meResponseJson`에 넘기면 앱의 `getMe()`가 읽을 JSON 응답이 만들어집니다.

```kotlin
import dev.rustinside.sample_rust_coresdk.meResponseJson
import org.springframework.http.MediaType
import org.springframework.http.ResponseEntity

fun profileResponse(id: String, handle: String, nickname: String): ResponseEntity<String> =
    ResponseEntity.ok()
        .contentType(MediaType.APPLICATION_JSON)
        .body(meResponseJson(id, handle, nickname))
```

`mePath()`와 `meResponseJson()`은 Rust에서 `#[rust_inside::server]`를 붙인 모듈에 있어 서버 SDK에서만 호출할 수 있습니다. 다른 계정 서버에 요청을 보낼 때는 서버에서도 `AccountClient.getMe()`를 쓸 수 있습니다.

## 로컬 테스트

Rust stable을 설치하고 라이브러리 루트에서 테스트를 실행하세요. 통합 테스트는 로컬 HTTP 서버를 띄웁니다.

```sh
cargo test --locked
```

## SDK 배포

### 첫 배포 전 설정

`rust-inside.json`에는 공개 API를 담은 Rust 모듈과 SDK를 배포할 저장소 다섯 곳을 지정했습니다. 새 배포처를 연결하려면 해당 저장소의 기본 브랜치에 첫 커밋을 만들고 설정 파일에 주소를 추가하세요. [배포 설정](rust-inside.json)

여러 저장소에 배포하려면 코어 저장소의 **Settings → Secrets and variables → Actions**에 `RUST_INSIDE_RELEASE_TOKEN`을 등록하세요. 대상 SDK 저장소에 **Contents: Read and write** 권한이 있는 토큰을 사용합니다. 공개된 Rust Inside 저장소를 읽을 때는 워크플로의 기본 GitHub 토큰을 사용합니다. [Rust Inside](https://github.com/Rust-Inside/rust-inside/blob/main/README.ko.md)

### 새 버전 배포

**Actions → Release sample SDKs → Run workflow**를 선택하면 `0.0.0`으로 빌드와 패키징을 시험할 수 있습니다. [릴리스 워크플로](.github/workflows/release.yml)

배포할 때는 코어의 릴리스 커밋에 버전 태그를 붙여 푸시하세요. 예를 들어 다음 버전 `0.1.1`을 배포하려면 아래 명령을 실행합니다.

```sh
git tag v0.1.1
git push origin v0.1.1
```

빌드가 성공하면 각 SDK 저장소의 **Releases**에서 새 버전을 확인할 수 있습니다.

## 라이선스

[MIT](LICENSE)
