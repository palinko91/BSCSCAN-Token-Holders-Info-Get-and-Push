# Purpose of this program

I've made this web scraper so you can use it to get the holder's amount from BSCscan and it will upload for you in JSON format to the chosen FTP.
From that point you can do with the data whatever you want and display to your site for example.

## Set the .env file before you run

It's important to set up the .env file by the given template, otherwise the program won't work.
You can set up the .env file in the / directory, same where is your Cargo.toml file too.

```
# .env

TOKEN_LINK=https://bscscan.com/token/generic-tokenholders2?m=normal&a=YOUR TOKEN'S ADDRESS
# You can set it the name so its gonna modify the json name creation
TOKEN_NAME=YOUR TOKEN'S NAME

FTP_ADDRESS=FTP SERVER ADDRESS WITH PORT USUALLY :21
FTP_USER=USERNAME FOR THE FTP SERVER
FTP_PASSWORD=PASSWORD FOR THE FTP SERVER
# You can select which directory to upload at the FTP server
DIRECTORY_TO_UPLOAD=/DESIRED FOLDER FOR THE FILE UPLOAD
```

## Token link

You can make the token's link pretty easy, all you need is the token's address, same what you adding to your metamask.

## Token name

It's easier if you add it, it just change the look of the generated file it will be in this format:
TokenName_Holders_ExactTimeGenerated

## About the FTP

You can simply set up the FTP login if you have the web server already.
Important to note you need to add the port for the address, usually that's :21
You should have permissions on that FTP server with the username and password combo you logging in.
And have to create the directory before you set up the directory to upload, because the program not creating directory for you.

For example if you want the 'holders' directory you login to your FTP server with Filezilla, creating the directory and giving '/holders' for the 'DIRECTORY_TO_UPLOAD' environment variable.

In that case the program after being run will upload the generated scraped file to that directory.

## Important to note

This version not have too much error handling, because I created the program for myself, so I'm wishing good luck!
Maybe we can improve it together?

## Contribution

Feel free to contribute, contact me on Discord palinko#0134 and we can figure out something.


