FastComments SDKをアプリの `build.gradle.kts` ファイルに追加します:

```kotlin
dependencies {
    implementation("com.fastcomments:android-sdk:2.1.0")
}
```

プロジェクトの `settings.gradle.kts` にRepsyリポジトリが含まれていることを確認してください:

```kotlin
dependencyResolutionManagement {
    repositories {
        maven {
            url = uri("https://repo.repsy.io/mvn/winrid/fastcomments")
        }
        // other repositories...
    }
}
```