var events = []

var activityVerbs = {
  Feeding: 'fed',
  Petting: 'pet',
  Playing: 'played with',
  Talking: 'talked to',
}

var activityUrls = {
  Feeding: 'feed',
  Petting: 'pet',
  Playing: 'play',
  Talking: 'talk',
}

function eventString(ev) {
  return (ev.human || 'Someone') + ' ' + activityVerbs[ev.activity] +
         ' Bronson ' + moment(ev.time).fromNow()
}

function getLatestFeeding() {
  return _(events)
    .filter(function(ev) { return ev.activity === 'Feeding' })
    .maxBy(function(ev) { return ev.time })
}

var hungerMessages = [
  "just ate! (Don't believe his lies!)",
  'is definitely not hungry.',
  'is still not hungry.',
  "is chillin'.",
  'is all good.',
  "is bein' cute.",
  'is thinking about food.',
  'is starting to get hungry.',
  'has the munchies.',
  'is getting hungrier by the minute.',
  'is really quite hungry.',
  'is letting you know about his hunger.',
  'probably should be fed about now.',
  'wants nothing more than foods!',
  'has needs! Food-related needs!',
  'is SO HUNGRY.',
  'needs food, badly!',
  'wants chicken! Bronson wants liver!',
  'has got to eat!',
  "will eat YOU if you don't feed him now!",
]

function hungerString() {
  var latestFeeding = getLatestFeeding().time
  var hoursSinceFeeding = moment(latestFeeding).diff(moment(), 'hours')
  if (hoursSinceFeeding >= 20) return 'Bronson is STARVING'
  if (hoursSinceFeeding < 0) hoursSinceFeeding = 0
  return 'Bronson ' + hungerMessages[hoursSinceFeeding]
}

function view() {
  return m('.container',
    m('h1', hungerString()),

    m('.tiles',
      m('.tile',
        m('h2', 'Pay tribute to Bronson'),
        m('input', {
          value: localStorage.human || '',
          onchange: m.withAttr('value', function(value) {
            localStorage.human = value
          }),
          placeholder: 'Enter your name here'
        }),
        Object.keys(activityUrls).map(function(activity) {
          return m('a.btn',
            {
              onclick: function(e) {
                var url = activityUrls[activity]
                var human = localStorage.human
                if (human) url += '?human=' + encodeURIComponent(human)
                m.request({ url: url }).then(fetchData)
              }
            },
            'I ' + activityVerbs[activity] + ' Bronson'
          )
        })
      ),

      m('.tile',
        m('h2', 'Bronson Activity Feed'),
        events.slice().reverse().map(function(ev) {
          return m('p.' + ev.activity, eventString(ev))
        })
      )
    )
  )
}

function fetchData() {
  return m.request({ url: 'events' })
    .then(function(response) { events = response })
}

fetchData().then(function() {
  m.mount(document.getElementById('app'), { view: view })
  setInterval(m.redraw, 60000)
})
