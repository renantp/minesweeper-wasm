import init, {greet,isLost, newField, getState, openFields, toggleFlag} from './pkg/minesweeper_lib.js'

    async function main(){
      await init();
      greet("Renan<renannguyen@pm.me>");
      render();
      let restartBtn = document.getElementById("restartBtn");
      restartBtn.addEventListener("click", evt =>{
        console.log(newField(10, 10, 20));
        render();
      });
    }
    function render() {
      let data = getState().split("\n").map(row => row.trim().split(/\s+/));
      console.log(data);
      let root = document.getElementById("root");
      root.innerText = "";
    //   root.style.display = "inline-grid";
      root.style.gridTemplate = `repeat(${data.length}, auto) / repeat(${data[0].length}, auto)`;

      for ( let y=0;y < data.length; y++ ){
        for ( let x = 0; x< data[y].length; x++ ){
          let element = document.createElement("a");
          element.classList.add("field");
          element.href = "#";
          element.innerText = data[y][x];

          element.addEventListener("click", evt=> {
            evt.preventDefault();
            console.log(x, y);
            openFields(x, y);
            render();
          });

          element.addEventListener("contextmenu", evt => {
            evt.preventDefault();

            toggleFlag(x, y);
            render();
          });

          root.appendChild(element);
        }
      }
      if(isLost()) document.getElementById("gameTitle").innerText = "Finish"; else document.getElementById("gameTitle").innerText = "Find the mines!"
    }
    main();