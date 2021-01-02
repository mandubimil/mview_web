webix.protoUI({
	name:"ckeditor",
	$init:function(config){
		this.$view.className += " webix_selectable";
		this._waitEditor = webix.promise.defer();
	},
	defaults:{
		borderless:true,
		language:"en",
		barHeight:44,
	},
	_init_ckeditor_once:function(){
		var tid = this.config.textAreaID = "t"+webix.uid();
		this.$view.innerHTML = "<textarea id='"+tid+"'>"+this.config.value+"</textarea>";
		
		if (this.config.cdn === false){
			this._render_ckeditor;
			return;
		};

		var cdn = this.config.cdn || "//cdn.ckeditor.com/4.9.2/standard/";
		
		window.CKEDITOR_BASEPATH = cdn;			
		webix.require([cdn+"/ckeditor.js"])
		.then( webix.bind(this._render_ckeditor, this) )
		.catch(function(e){
			console.log(e);
		});		
	},
	_render_ckeditor:function(){
		var initMethod = "replace";
		if(this.config.editorType === "inline") {
			CKEDITOR.disableAutoInline = true;
			initMethod = "inline";
		}
		
		this._3rd_editor = CKEDITOR[initMethod]( this.config.textAreaID, {
			language: this.config.language,
			width:this.$width -2,
			height:this.$height - this.config.barHeight,
			toolbarGroups : [
				{ name: 'clipboard', groups: [ 'clipboard', 'undo' ] },
				{ name: 'editing', groups: [ 'find', 'selection', 'spellchecker', 'editing' ] },
				{ name: 'links', groups: [ 'links' ] },
				{ name: 'insert', groups: [ 'insert' ] },
				{ name: 'forms', groups: [ 'forms' ] },
				{ name: 'tools', groups: [ 'tools' ] },
				{ name: 'document', groups: [ 'mode', 'document', 'doctools' ] },
				{ name: 'others', groups: [ 'others' ] },
				{ name: 'basicstyles', groups: [ 'basicstyles', 'cleanup' ] },
				{ name: 'paragraph', groups: [ 'list', 'indent', 'blocks', 'align', 'bidi', 'paragraph' ] },
				{ name: 'styles', groups: [ 'styles' ] },
				{ name: 'colors', groups: [ 'colors' ] },
				{ name: 'about', groups: [ 'about' ] }		
			],
			removeButtons : 'Subscript,Superscript,Cut,Copy,Paste,PasteText,PasteFromWord,Redo,Undo,Scayt,Unlink,Anchor,Link,Image,SpecialChar,Maximize,Source,RemoveFormat,Outdent,Indent,Blockquote,About,Styles'
		});
		this._waitEditor.resolve(this._3rd_editor);
	},
	_set_inner_size:function(x, y){
		if (!this._3rd_editor || !this._3rd_editor.container || !this.$width || this.config.editorType === "inline") return;
		this._3rd_editor.resize(x, y);
	},
	$setSize:function(x,y){
		if (webix.ui.view.prototype.$setSize.call(this, x, y)){
			this._init_ckeditor_once();
			this._set_inner_size(x,y);
		}
	},
	setValue:function(value){
		this.config.value = value;

		if (this._3rd_editor && this._3rd_editor.status === "ready")
			this._3rd_editor.setData(value);
		else webix.delay(function(){
			this.setValue(value);
		},this,[],100);
	},
	getValue:function(){
		return this._3rd_editor?this._3rd_editor.getData():this.config.value;
	},
	focus:function(){
		this._focus_await = true;
		if (this._3rd_editor)
			this._3rd_editor.focus();
	},
	getEditor:function(waitEditor){
		return waitEditor?this._waitEditor:this._3rd_editor;
	}
}, webix.ui.view);
