plugins {
    id("com.android.application")
    id("org.jetbrains.kotlin.android")
}

android {
    namespace="com.sohnidas_studios.boonk_arbeitszeit"
    compileSdk = 33
    signingConfigs {
    create("release") {
        storeFile = file("keystore.jks") // Use '=' instead of space
        storePassword = "ernaerna"
        keyAlias = "Glamund"
        keyPassword = "ernaerna"
    }
}
    defaultConfig {
        applicationId = "com.sohnidas_studios.boonk_arbeitszeit"
        minSdk = 24
        targetSdk = 33
        versionCode = 1
        versionName = "1.0"
    }
    buildTypes {
        getByName("debug") {
            isDebuggable = true
            isJniDebuggable = true
            isMinifyEnabled = false
            packaging {
                jniLibs.keepDebugSymbols.add("*/arm64-v8a/*.so")
                jniLibs.keepDebugSymbols.add("*/armeabi-v7a/*.so")
                jniLibs.keepDebugSymbols.add("*/x86/*.so")
                jniLibs.keepDebugSymbols.add("*/x86_64/*.so")
            }
        }
        getByName("release") {
            isMinifyEnabled = true
             proguardFiles(
                *fileTree(".") { include("**/*.pro") }
                    .plus(getDefaultProguardFile("proguard-android-optimize.txt"))
                    .toList().toTypedArray()
            )
            signingConfig = signingConfigs.getByName("release")
        }
    }
    kotlinOptions {
        jvmTarget = "1.8"
    }
    buildFeatures {
        buildConfig = true
    }
}

dependencies {
  implementation("androidx.core:core:1.9.0")

    implementation("androidx.webkit:webkit:1.6.1")
    implementation("org.jetbrains.kotlinx:kotlinx-coroutines-android:1.7.2")
    implementation("androidx.appcompat:appcompat:1.6.1")
    implementation("com.google.android.material:material:1.8.0")
}
