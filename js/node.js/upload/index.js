const path = require('path')
const express = require('express')
const multer = require('multer')

const app = express()
const port = 3000

const upload = multer({ dest: 'uploads/' })

app.get('/upload', (req, res) => res.sendFile(path.join(__dirname, 'upload.html')))

app.post('/upload', upload.array('file'), function (req, res) {
  res.send('' + req.files.length + ' files uploaded\n');
})

app.listen(port,function(){
  console.log(`listening on port ${port}!`)
})	
