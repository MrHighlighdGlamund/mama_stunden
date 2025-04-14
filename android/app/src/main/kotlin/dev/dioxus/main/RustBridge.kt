// package com.example.DxTest

package dev.dioxus.main
import android.content.Intent
import android.widget.Toast
import android.app.Activity
import android.content.pm.PackageManager
import androidx.core.content.ContextCompat
import android.provider.Settings
import androidx.appcompat.app.AlertDialog


class RustBridge {

    companion object {
        // Load the Rust library
        init {
            System.loadLibrary("dioxusmain")  // Make sure this matches the name of your Rust library
        }
    }

    // This is the JNI method that will be called from Kotlin to trigger Rust code
    external fun callRustFunction(message: String)

    
}

