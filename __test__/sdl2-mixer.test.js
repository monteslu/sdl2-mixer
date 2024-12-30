import test from 'ava'
import { Mixer } from '../index.js'
import path from 'path'

const sleep = ms => new Promise(resolve => setTimeout(resolve, ms))

let mixer
const resourcePath = path.join('__test__', 'resources')

test.before(() => {
  mixer = new Mixer()
})

test('Mixer can be instantiated', t => {
  t.truthy(mixer)
})

test('Mixer methods exist', t => {
  t.is(typeof mixer.loadWav, 'function')
  t.is(typeof mixer.loadMusic, 'function')
  t.is(typeof mixer.playChannel, 'function')
  t.is(typeof mixer.playMusic, 'function')
  t.is(typeof mixer.haltChannel, 'function')
  t.is(typeof mixer.haltMusic, 'function')
  t.is(typeof mixer.volumeMusic, 'function')
  t.is(typeof mixer.volumeChunk, 'function')
})

test('Sound loading fails with nonexistent file', async t => {
  const error = t.throws(() => mixer.loadWav('nonexistent.wav'))
  t.truthy(error)
})

test('Music loading fails with nonexistent file', async t => {
  const error = t.throws(() => mixer.loadMusic('nonexistent.mp3'))
  t.truthy(error)
})

test('Can load and play WAV file', async t => {
  const chunk = mixer.loadWav(path.join(resourcePath, 'explosion.wav'))
  t.truthy(chunk)
  
  mixer.volumeChunk(0, 128)
  mixer.playChannel(chunk, 0, 0)
  await sleep(500)
  
  mixer.volumeChunk(0, 255)
  mixer.playChannel(chunk, 0, 0)
  await sleep(500)
  
  mixer.haltChannel(0)
  t.pass()
})

test('Can load and play Music file', async t => {
  const music = mixer.loadMusic(path.join(resourcePath, 'song.mp3'))
  t.truthy(music)
  
  const originalVolume = mixer.volumeMusic(128)
  mixer.playMusic(music, 0)
  await sleep(1000)
  
  mixer.volumeMusic(255)
  await sleep(1000)
  
  mixer.haltMusic()
  mixer.volumeMusic(originalVolume)
  t.pass()
})

test('Can play multiple sound channels', async t => {
  const chunk = mixer.loadWav(path.join(resourcePath, 'explosion.wav'))
  t.truthy(chunk)
  
  mixer.playChannel(chunk, 0, 0)
  await sleep(100)
  mixer.playChannel(chunk, 1, 0)
  await sleep(100)
  mixer.playChannel(chunk, 2, 0)
  await sleep(500)
  
  mixer.haltChannel(0)
  mixer.haltChannel(1)
  mixer.haltChannel(2)
  t.pass()
})

test('Music looping works', async t => {
  const music = mixer.loadMusic(path.join(resourcePath, 'song.mp3'))
  t.truthy(music)
  
  mixer.playMusic(music, -1)
  await sleep(1000)
  mixer.haltMusic()
  t.pass()
})

test('Can mix multiple sounds simultaneously', async t => {
  // Start background music at 50% volume
  const music = mixer.loadMusic(path.join(resourcePath, 'song.mp3'))
  mixer.volumeMusic(128)
  mixer.playMusic(music, -1) // loop infinitely
  
  // Load explosion sound
  const explosion = mixer.loadWav(path.join(resourcePath, 'explosion.wav'))
  
  // Play explosions on different channels while music is playing
  mixer.playChannel(explosion, 0, 0)
  await sleep(200) // slight delay
  mixer.playChannel(explosion, 1, 0)
  await sleep(200)
  mixer.playChannel(explosion, 2, 0)
  await sleep(1000) // let them finish
  
  // Play rapid explosions
  mixer.playChannel(explosion, 3, 0)
  mixer.playChannel(explosion, 4, 0)
  mixer.playChannel(explosion, 5, 0)
  await sleep(1000)
  
  // Clean up
  mixer.haltMusic()
  for (let i = 0; i < 6; i++) {
    mixer.haltChannel(i)
  }
  
  t.pass()
})