import './App.css';
import { useState, useEffect } from 'react';

const PAGE_QUANTITY = 20;

function App() {
  // Hooks
  const [picturesList, setPicturesList] = useState([]);
  const [videoList, setVideoList] = useState([]);
  const [anecdotesList, setAnecdotesList] = useState([]);
  const [condolencesList, setCondolencesList] = useState([]);
  const [headerText, setHeaderText] = useState("");
  const [pageContent, setPageContent] = useState(
      <div>
        Loading...
      </div>
    );

  function fetchList(route, setValueFun) {
    fetch(route)
    .then(response => response.json())
    .then((resultJsonData) => {
      setValueFun(resultJsonData);
      console.log(resultJsonData);
    })
    .catch((error) => {
      console.error(error)
    })
  }

  // Load the data from backend
  useEffect(() => {
    fetchList('/pictureList', setPicturesList)

    fetchList('/videoList', setVideoList)  

    fetchList('/anecdoteList', setAnecdotesList)
  
    fetchList('/condolenceList', setCondolencesList)
  
  }, []);

  // Set Anecdotes as initial page
  useEffect(() => {
    setHeaderText("Memories")
    setPageContent(
      <AnecdotePage
        anecdotesList={anecdotesList}
      />
    )
  }, [anecdotesList]);

  function NavButton({text, content}) {
    return (
      <button class="nav-button" onClick={() => {
          setPageContent(
            content
          )
          setHeaderText(text)
        }
      }>
        {text}
      </button>
    )
  }
  
  return (
    <div>
      <header>
        <div class="title">{headerText}</div>
        <div class="subtitle">Billy Ring 2001-2023</div>
      </header>
      <nav>
        <NavButton
          text={"Gallery"}
          content={(
            <PicturePage
              pictureList={picturesList}
            />
          )} 
        />
        <NavButton
          text={"Videos"}
          content={(
            <VideoPage
              videoList={videoList}
            />
          )} 
        />
        <NavButton
          text={"Memories"}
          content={(
            <AnecdotePage
              anecdotesList={anecdotesList}
            />
          )} 
        />
        <NavButton
          text={"Condolences"}
          content={(
            <CondolencePage
              condolencesList={condolencesList}
            />
          )} 
        />
      </nav>
      <main>
        {pageContent}
      </main>
    </div>
  );
}



function PicturePage({pictureList}) {
  
  function Picture({picture}) {
    return (
      <figure key={picture.picture_id} class="pictureFrame">
        <img src={`imageFiles/${picture.filename}`} alt='missing' class="picture" />
        <figcaption>{picture.caption}</figcaption>
      </figure>
    )  
  }
  
  const [pictureIndex, setPictureIndex] = useState(0);
  const pictureItems = pictureList.slice(pictureIndex, pictureIndex+PAGE_QUANTITY).map(picture =>
    <Picture
      picture={picture} 
    />
  );

  return (
    <div>
      <PageIndexSelector list_size={pictureList.length} changePageFunction={setPictureIndex} current_index={pictureIndex}/>
      <div class="pictureGrid">
        {pictureItems}
      </div> 
      <PageIndexSelector list_size={pictureList.length} changePageFunction={setPictureIndex} current_index={pictureIndex}/>
    </div>
  )  
}



function VideoPage({videoList}) {
  
  function VideoLink({video}) {
    return (
      <div key={video.video_id} class="pictureFrame">
        <a href={video.url} class="picture">{video.title}</a> 
      </div>
    )  
  }
  
  const [videoIndex, setVideoIndex] = useState(0);
  const videoItems = videoList.slice(videoIndex, videoIndex+PAGE_QUANTITY).map(video =>
    <VideoLink
      video={video} 
    />
  );

  return (
    <div>
      <PageIndexSelector list_size={videoList.length} changePageFunction={setVideoIndex} current_index={videoIndex}/>
      <div class="pictureGrid">
        {videoItems}
      </div> 
      <PageIndexSelector list_size={videoList.length} changePageFunction={setVideoIndex} current_index={videoIndex}/>
    </div>
  )  
}



function AnecdotePage({anecdotesList}) {
  
  function Anecdote({anecdote}) {
    return (
      <div key={anecdote.anecdote_id} class="anecdote">
        {anecdote.content}
        <br />
        <br />
        <div class="author">{anecdote.author}</div>
      </div>
    )  
  }

  function AnecdoteTextBox() {
    return (
      <div class="anecdote">
        <form method="post" action="/newAnecdote" >
          <label for="author">Your name:</label><br />
          <input type="text" id="author" name="author" placeholder='Your Name' required class="authorField" /><br />
          <br />
          <label for="content">Memory:</label><br />
          <textarea name="content" id="content" rows="10" required placeholder='Memory' class="contentField"/>
          <br />
          <br />
          <input type="submit" value="Submit" class="submitButton" />
        </form> 
      </div>
    )  
  }

  const [anecdoteIndex, setAnecdoteIndex] = useState(0);
  const anecdoteItems = anecdotesList.slice(anecdoteIndex, anecdoteIndex+PAGE_QUANTITY).map(anecdote =>
    <Anecdote
      anecdote={anecdote} 
    />
  );

  return (
    <div>
      <PageIndexSelector list_size={anecdotesList.length} changePageFunction={setAnecdoteIndex} current_index={anecdoteIndex}/>
      {anecdoteItems}
      <PageIndexSelector list_size={anecdotesList.length} changePageFunction={setAnecdoteIndex} current_index={anecdoteIndex}/>
      <AnecdoteTextBox />
    </div>
  )  
}



function CondolencePage({condolencesList}) {
  
  function Condolence({condolence}) {
    return (
      <div key={condolence.condolence_id} class="condolence">
        {condolence.content}
        <br />
        <br />
        <div class="author">{condolence.author}</div>
      </div>
    )  
  }
  
  const [condolenceIndex, setCondolenceIndex] = useState(0);
  const condolenceItems = condolencesList.slice(condolenceIndex, condolenceIndex+PAGE_QUANTITY).map(condolence =>
    <Condolence
      condolence={condolence} 
    />
  );

  return (
    <div>
      <PageIndexSelector list_size={condolencesList.length} changePageFunction={setCondolenceIndex} current_index={condolenceIndex}/>
      {condolenceItems}
      <PageIndexSelector list_size={condolencesList.length} changePageFunction={setCondolenceIndex} current_index={condolenceIndex}/>
    </div>
  )  
}



function PageIndexSelector({list_size, changePageFunction, current_index}) {
  const number_of_pages = Math.ceil(list_size/PAGE_QUANTITY)
  const current_page = current_index/PAGE_QUANTITY + 1
  
  const decrementButton = (
      <button onClick={() => 
        changePageFunction(current_index-PAGE_QUANTITY)
      }>
        {'←'}
      </button>
  )
  
  const incrementButton = (
      <button onClick={() => 
        changePageFunction(current_index+PAGE_QUANTITY)
      }>
        {'→'}
      </button>
  )
  
  return (
    <div class="pageIndexSelector">
      {current_page > 1 && decrementButton}
      {current_page+" / "+number_of_pages}
      {current_page < number_of_pages && incrementButton}
    </div>
  )
}

export default App;
