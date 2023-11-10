# Otto

A repo for the project used to create a Non-Apps with interaction completely through users' native messaging apps.

![Otto Squid - the Auto Messenging Client](docs/squid.png)

## Background

This project was born from the fact that, everytime I buy some new device (like a Ninebot scooter), I have to install a new app for what amounts to a one-time config. It is tedious to download the app, create the account, and then wonder if I can delete the app or not.

The idea behind auto is to build the scaffolding that will enable device manufacturers, local service providers, and small businesses to create a messaging-based interface for their products and services.

- Config your device from your message client via text
- Make a reservation at a restaurant or salon with a text
- Order cookies from the neighbor's home-front kitchen with a text

And the provider won't have to engage at all. Instead, they will see the output of the auto-messaging systems via their preferred delivery system (db, email, text, etc).

Also, I wanted to learn Rust, so this project is as much about learning as anything else.

## How it works

The idea is for a customer to be on-boarded and to be able to set up their workflow completely via text (thereby getting an understanding of the flow). And for their services to be made available natively in the Google Maps and Apple Maps chat options, as well as via a dedicated phone number. It will likely rely on Sinch as a backend messaging provider.
