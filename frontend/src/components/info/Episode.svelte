<script lang="ts">
    export let epi;
    export let image;
    export let title;
    export let time;
    import { subscribedrag, updateDragOverIndex, updateDragStartIndex } from "./stores/localstore";
    import { changeOrder } from "./stores/episodes_store";


    let item;
    let dragIndexes;
    subscribedrag(value => dragIndexes = value)

    $: genEpi = () => {
        let addtoend = 1;
        if (dragIndexes.dragStartIndex == epi) return "  ";
        if (dragIndexes.dragStartIndex != null && dragIndexes.dragStartIndex < epi) {
            addtoend--;
        }
        if (dragIndexes.dragOverIndex != null && dragIndexes.dragOverIndex <= epi) {
            addtoend++
        }
        return epi + addtoend + ". ";
    }
    const generateId = (id: number) => {
        let addtoend = 1;
        if (dragIndexes.dragStartIndex == id) return "  ";
        if (dragIndexes.dragStartIndex != null && dragIndexes.dragStartIndex < id) {
            addtoend--;
        }
        if (dragIndexes.dragOverIndex != null && dragIndexes.dragOverIndex <= id) {
            addtoend++
        }
        return id + addtoend + "."
    }

    const onDragStart = (e: any) => {
        //create ghostNode
        let ghostNode = e.target.cloneNode(true)

        //delete default
        let image = new Image(0, 0);
        image.src = 'data:image/gif;base64,R0lGODlhAQABAIAAAAAAAP///yH5BAEAAAAALAAAAAABAAEAAAIBRAA7';
        e.dataTransfer.setDragImage(image, 0, 0);
        e.dataTransfer.effectAllowed = 'move';

        //setup ghostnode
        ghostNode.style.position = "absolute";

        //change id
        ghostNode.childNodes[0].childNodes[0].textContent = generateId(epi)

        //set position
        ghostNode.style.top = (e.pageY - e.target.offsetHeight / 2 - e.target.offsetHeight * 0.8 / 2) + 'px'
        ghostNode.style.left = (e.pageX - e.target.offsetWidth / 2) + 'px'

        //set width and height
        ghostNode.style.height = e.target.offsetHeight + 'px';
        ghostNode.style.width = e.target.offsetWidth + 'px';

        //styling ghostndoe
        ghostNode.style.zIndex = 1000;
        ghostNode.style.opacity = 0.8;
        ghostNode.style.scale = .8;
        ghostNode.style.pointerEvents = 'none';
        ghostNode.id = 'ghostNode';

        //apend
        document.body.prepend(ghostNode)
        //disable selected
        if (item) {
            item.classList.add('dragstart');
        }

        if (dragIndexes.dragStartIndex == null) {
            updateDragStartIndex(epi);
        }
        if (dragIndexes.dragOverIndex == null) {
            updateDragOverIndex(epi);
            let ghostNode: HTMLElement | null = document.querySelector('#ghostNode');
            if (ghostNode) {
                ghostNode.childNodes[0].childNodes[0].textContent = generateId(epi)
            }
        }
    }

    const onDrag = (e: any) => {
        if (dragIndexes.dragStartIndex == null) return;
        let ghostNode: HTMLElement | null = document.querySelector('#ghostNode');
        if (ghostNode) {
            ghostNode.style.top = (e.pageY - e.target.offsetHeight / 2 - e.target.offsetHeight * 0.8 / 2) + 'px'
            ghostNode.style.left = (e.pageX - e.target.offsetWidth / 2) + 'px'
        }

    }

    const onDragEnd = () => {
        if (dragIndexes.dragStartIndex == null) return;
        let ghostNode: HTMLElement | null = document.querySelector('#ghostNode');
        if (ghostNode) {
            ghostNode.remove();
        }

        if (item) {
            item.classList.remove('dragstart');
        }

        if (dragIndexes.dragStartIndex != null) {
            updateDragStartIndex(null);
        }

        if (dragIndexes.dragOverIndex != null) {
            updateDragOverIndex(null);
        }

    }

    const onDragEnter = () => {
        if (dragIndexes.dragStartIndex == null) return;
        if (item) {
            item.classList.add('dragover');
        }
        if (dragIndexes.dragOverIndex == null || dragIndexes.dragOverIndex != epi) {
            updateDragOverIndex(epi);
            let ghostNode: HTMLElement | null = document.querySelector('#ghostNode');
            if (ghostNode) {
                if (epi == dragIndexes.dragStartIndex) {
                    ghostNode.childNodes[0].childNodes[0].textContent = epi + 1 + ". "
                } else if (dragIndexes.dragStartIndex && dragIndexes.dragStartIndex > epi) {
                    ghostNode.childNodes[0].childNodes[0].textContent = epi + 1 + ". "
                } else {
                    ghostNode.childNodes[0].childNodes[0].textContent = epi + ". "
                }
            }
        }
    }

    const onDragLeave = () => {
        if (dragIndexes.dragStartIndex == null) return;
        if (item) {
            item.classList.remove('dragover');
        }
    }

    const onDrop = () => {
        if (dragIndexes.dragStartIndex == null) return;
        if (item) {
            item.classList.remove('dragover');
        }
        if (dragIndexes.dragStartIndex) {
            changeOrder(dragIndexes.dragStartIndex, epi);
        }
    }

    const onDragOver = () => {
        if (dragIndexes.dragStartIndex == null) return;
        if (item && item.classList.contains('dragover')) {
            item.classList.add('dragover');
        }

    }

</script>
<div class="item" bind:this={item}
     draggable={true}
     on:dragstart={onDragStart}
     on:dragend={onDragEnd}
     on:drag={onDrag}
     on:drop={onDrop}
     on:dragover={onDragOver}
     on:dragenter={onDragEnter}
     on:dragleave={onDragLeave}
>
    <div class="icon"><img draggable="false" src={image} alt="Image"></div>
    <div class="title"><h1>{genEpi() + title}</h1>
        <text>{time}</text>
    </div>
</div>
<style lang="scss">
  .item {
    display: flex;
    width: 100%;
    padding: 2%;
    background: rgba(0, 0, 0, 0.5);
    border-radius: 5px;
    height: 5vh;

    & .icon {
      height: 100%;

      & img {
        height: 100%;
      }
    }
  }
</style>