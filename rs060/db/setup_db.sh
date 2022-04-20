#!/usr/bin/env bash
cd $(dirname "$0")
sqlite3 ../target/weather.db < db.sql
sqlite3 -csv ../target/weather.db ".import nyc_centralpark_weather.csv nyc_weather"