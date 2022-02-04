<script lang="ts">
	import { onMount } from 'svelte';
	import UIkit from 'uikit'
	import Icons from 'uikit/dist/js/uikit-icons'
	import 'uikit/dist/css/uikit.css'
	import 'uikit/dist/css/uikit.min.css'
	UIkit.use(Icons);

	import { invoke } from '@tauri-apps/api/tauri'


	/* タブ関係処理 */
	let tabs_ = ['C:/'];
	let paincontents_ = [];
	invoke('get_content_from_path', {dir: tabs_[0]}).then((msg) => paincontents_.push(msg));
	let tabs = ['tab1', 'Tab2', 'tab3'];
	let paincontents = ['cont1', 'Cont2', 'cont3'];
  const closeTab = (i) => {
		tabs.splice(i, 1);
		tabs = tabs;
	}

	const addTab = () => {
		tabs = [...tabs, `tab${tabs.length+1}`];
		paincontents = [...paincontents, `cont${paincontents.length+1}`];
	};

	onMount(async () => {
		UIkit.util.on("#tab-ul", "moved", (e) => {
			const to = [...e.srcElement.childNodes].map(item=>item.id).indexOf(e.detail[1].id);
			const from = e.detail[0].origin['index'];
			if (to >= paincontents.length) {
        var k = to - paincontents.length + 1;
        while (k--) { paincontents.push(undefined) }
			}
			paincontents.splice(to, 0, paincontents.splice(from, 1)[0]);
			paincontents = paincontents;
			console.log(paincontents_)

			UIkit.switcher("#main-pain").show(2);
		});
	});

	/* サイド幅可変処理 */
	let w_dividerX = 1;
	const onSideOver = (e) => { w_dividerX = 5; }
	const onSideLeave = (e) => { w_dividerX = 1; }

	let currentX;
	let onSideDown = (e) => {
		currentX = e.x;
		document.addEventListener("mousemove", resizeSide, false);
	};

	document.addEventListener("mouseup", ()=>document.removeEventListener("mousemove", resizeSide, false));

	$: sideWidth = 150;
	const resizeSide = (e) => {
		sideWidth -= (currentX - e.x);
		currentX = e.x
	}

	/* サイド表示物設定 */
	const sideIitems = [
		["favorite.ico", "お気に入り"], ["pc.ico", "ドライブ"], ["network.ico", "ネットワーク"]
	];

</script>

<container>
	<section id="side" style="width: {sideWidth}px;">
		<ul class="uk-list" uk-sortable="handle: .side-items" uk-accordion="multiple: true">
			{#each sideIitems as item}
			<li class="side-items">
        <!-- svelte-ignore a11y-missing-attribute -->
        <a class="uk-accordion-title"><img src="icons/{item[0]}"> {item[1]}</a>
        <div class="uk-accordion-content">
					<p>hoge</p>
        </div>
    	</li>
			{/each}
		</ul>

		<!-- svelte-ignore a11y-mouse-events-have-key-events -->
		<div id="dividerX"
			on:mouseover="{onSideOver}"
			on:mouseleave="{onSideLeave}"
			on:mousedown="{onSideDown}"
			style="width: {w_dividerX}px">
		</div>
	</section>

	<div id="tabmain">
		<section id="tab">
			<ul id="tab-ul" uk-tab="connect: #main-pain" uk-sortable="handle: .sortable-handle">
				{#each tabs as tab, i}
				<!-- svelte-ignore a11y-invalid-attribute -->
				<li id="{i.toString(10)}">
					<a href="#">
						<span class="sortable-handle uk-margin-small-right uk-text-center" uk-icon="icon: table"></span>
						{tab}
						<button uk-icon="close" on:mousedown={()=>{closeTab(i)}}></button>
					</a>
				</li>
				{/each}
			</ul>

			<button uk-icon="plus" on:mousedown="{addTab}"></button>
		</section>

		<section id="main">
			<ul id="main-pain" class="uk-switcher uk-margin">
				{#each paincontents as cont, i}
					<li>{cont}</li>
				{/each}
			</ul>
		</section>
	</div>
</container>

<footer>
</footer>

<style lang="scss">
	container {
		height: 100%;
		display: flex;
	}

	#side {
		position: relative;
		height: 100%;
		min-width: 30px;
		overflow: hidden;
	}

	#dividerX {
		content: '';
		position: absolute;
		background-color: lightgray;
		top: 0;
		right: 0;
		height: 100%;
		padding: 2px;
		background-clip: content-box;
		cursor: ew-resize;
	}

	#tabmain {
		flex: 1;
	}

	#tab {
		display: flex;

		a {
			text-transform: none;
		}

		button {
			margin-bottom: 15px;
		}
	}
</style>