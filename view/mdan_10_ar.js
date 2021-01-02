var cur_page = 1;
var jsd_display = [];

var time_id = null;
var mdan_backup = "";


function start_page()
{
	$$("input_text_dan_num").disable();
	read_config();    
}

function show_window_input()
{
	$$("input_text_dan_num").setValue("new");

	$$('input_combobox_group').setValue($$('main_combobox_group').getValue());
	$$('input_text_title').setValue("");
	$$('input_text_point').setValue("");
	$$('input_text_content').setValue("");
	$$('input_text_history').setValue("");

	$$("window_input").show();

	mdan_backup = $$('input_text_title').getValue()+$$('input_text_point').getValue()+$$('input_text_content').getValue()+$$('input_text_history').getValue();
	time_id = setInterval("check_update_mdan();", 30000);
}

function close_window_input()
{
	if (time_id != null)
	{
		clearInterval(time_id);
	}

	$$('window_input').hide();		
}

function check_update_mdan()
{
	if ($$('input_text_dan_num').getValue() != "new")
	{
		let mdan_now = $$('input_text_title').getValue()+$$('input_text_point').getValue()+$$('input_text_content').getValue()+$$('input_text_history').getValue();
		if (mdan_backup != mdan_now)
		{
			var send_json = { 
				"para1":
				{
					"분류":$$('input_combobox_group').getValue(),                             
					"단어번호":$$('input_text_dan_num').getValue(), 
					"제목":$$('input_text_title').getValue(),
					"핵심어":$$('input_text_point').getValue(),
					"내용":$$('input_text_content').getValue(),
					"히스토리":$$('input_text_history').getValue(),
				}
			};    
			webix.ajax().headers({"Content-type":"application/json"}).post("/mdan_10_ar/update_dan_imsi", send_json, function(text)
			{		
				webix.message("임시저장 : "+text);
				mdan_backup = mdan_now
			});				
		}
	}
}

function show_window_config()
{
	$$("window_config").show();
	read_config();
}

function read_config()
{
	var send_json = {};
	webix.ajax().headers({"Content-type":"application/json"}).post("/mdan_10_ar/read_config", send_json, function(text)
	{		
		if ((text == '{"data":"none"}') || (text == '[{"설정":null}]'))
		{
			webix.message(text);
			return 0;
		}

		var jsd = JSON.parse(text)[0]["설정"];
		var jsd = JSON.parse(jsd);

		$$('main_combobox_group').setValue(jsd["기본분류"]);
		
		$$('config_combobox_group').setValue(jsd["기본분류"]);
		$$('config_combobox_display').setValue(jsd["출력"]);
		$$('config_text_div_num').setValue(jsd["분할"]);
		$$('config_text_page_num').setValue(jsd["페이지당"]);
	});		
}

function save_config()
{
	var send_json = { 
		"para1":
		{
			"설정":'{"기본분류":"'+$$('config_combobox_group').getValue()+'", "출력":"'+$$('config_combobox_display').getValue()+'", "분할":"'+$$('config_text_div_num').getValue()+'", "페이지당":"'+$$('config_text_page_num').getValue()+'"}'
		}
	};

	webix.ajax().headers({"Content-type":"application/json"}).post("/mdan_10_ar/save_config", send_json, function(text)
	{		
		webix.message(text);	
		$$("window_config").hide();	
		read_mdan_main("save_config");
	});	
}

function save_dan()
{
	var send_json = { 
                        "para1":
                        {
                            "분류":$$('input_combobox_group').getValue(),                             
                            "단어번호":$$('input_text_dan_num').getValue(), 
                            "제목":$$('input_text_title').getValue(),
                            "핵심어":$$('input_text_point').getValue(),
                            "내용":$$('input_text_content').getValue(),
                            "히스토리":$$('input_text_history').getValue(),
                        }
					};    
	if (($$('input_combobox_group').getValue() == "") || ($$('input_combobox_group').getValue() == null) || ($$('input_combobox_group').getValue() == undefined))
	{
		webix.message("분류를 선택하세요");
		return 1;
	}

	if ($$('input_text_dan_num').getValue() == "new")
		webix.ajax().headers({"Content-type":"application/json"}).post("/mdan_10_ar/save_dan", send_json, function(text)
		{		
			webix.message(text);		

			// $$("window_input").hide();
			$$('input_text_dan_num').setValue(text)
			read_mdan_main("new");
		});	
	else
		webix.ajax().headers({"Content-type":"application/json"}).post("/mdan_10_ar/update_dan", send_json, function(text)
		{		
			webix.message(text);		

			// $$("window_input").hide();
			read_mdan_main("update");
		});	
}

function test()
{
	$$("mdan_board").removeView("mdan_content");
	$$("mdan_board").addView({id:"mdan_content", rows:[]});
	$$("mdan_content").addView
	(
		{template:"jsd[i].내용1", autoheight:true}
	);
}

function read_mdan_main(job)
{
	var send_json = {
		para1 : 
		{
			"분류": $$('main_combobox_group').getValue()
		}		
	};	

	webix.ajax().headers({"Content-type":"application/json"}).post("/mdan_10_ar/read_mdan_all", send_json, function(text)
	{  
		$$("mdan_board").removeView("mdan_content");
		$$("mdan_board").addView({id:"mdan_content", rows:[]});

		if ( (job == "change_combobox") || (job == "save_config") )
			cur_page =1;

		if (text == '{"data":"none"}')
		{
			webix.message(text);
			return 0;
		}
	
		var div_num = parseInt($$('config_text_div_num').getValue());
		jsd_display = [];
		var jsd = JSON.parse(text);
		for ( i=0 ; i<jsd.length ; i++ )
		{
			if ((i%div_num) == 0)
			{
				jsd_display.push(jsd[i]);
			}
		}
		
		display_dan();
	});			
}

function del_dan()
{
	if ($$('input_text_dan_num').getValue() == "new")
		return 1;

	var send_json = { 
                        "para1":
                        {
                            "단어번호":$$('input_text_dan_num').getValue(), 
                        }
					};
					
	webix.confirm({title:"삭제!!", text:"정말 삭제 하시겠습니까?", callback:function(result)
	{
		if (result === true)
		{
			webix.ajax().headers({"Content-type":"application/json"}).post("/mdan_10_ar/del_dan", send_json, function(text)
			{		
				webix.message(text);		

				$$("window_input").hide();
				read_mdan_main("del");	
			});	
		}				
	}});	
}

function update_dan(p_dan_num, p_jsd_num)
{
	$$("input_text_dan_num").setValue(p_dan_num);
	$$("window_input").show();

	$$("input_combobox_group").setValue(jsd_display[p_jsd_num].분류);
	$$("input_text_title").setValue(jsd_display[p_jsd_num].제목);
	$$("input_text_point").setValue(jsd_display[p_jsd_num].핵심어);
	$$("input_text_content").setValue(jsd_display[p_jsd_num].내용);
	$$("input_text_history").setValue(jsd_display[p_jsd_num].히스토리);

	mdan_backup = $$('input_text_title').getValue()+$$('input_text_point').getValue()+$$('input_text_content').getValue()+$$('input_text_history').getValue();
	time_id = setInterval("check_update_mdan();", 30000);
}

function view_dan(dan_num, i)
{
	webix.message(jsd_display[i]["제목"]);
	$$("window_view").show();

	$$("mdan_board_view").removeView("mdan_content_view");
	$$("mdan_board_view").addView({id:"mdan_content_view", rows:[]});

	$$("mdan_content_view").addView
	(				
		{template:"<b>"+jsd_display[i].제목+"</b>", autoheight:true, borderless:true,}				
	);

	if (!(jsd_display[i].핵심어==null || jsd_display[i].핵심어==""))
	{
		$$("mdan_content_view").addView
		(
			{template:"<font color='900c3f'> - "+jsd_display[i].핵심어+"</font>", autoheight:true, borderless:true}
		);
	}

	$$("mdan_content_view").addView
	(
		{template:jsd_display[i].내용, autoheight:true, borderless:true}
	);

	$$("mdan_content_view").addView
	(
		{template:jsd_display[i].히스토리, autoheight:true, borderless:true}
	);
}

function display_dan()
{
	var page_num = parseInt($$('config_text_page_num').getValue());
	var div_num = parseInt($$('config_text_div_num').getValue());

	$$("mdan_board").removeView("mdan_content");
	$$("mdan_board").addView({id:"mdan_content", rows:[]});

	for ( let i=0 ; i<jsd_display.length ; i++ )
	{
		if ( (i >= (cur_page-1)*page_num) && ( i < (cur_page)*page_num) )
		{
			var temp_str_1 = "<a href='#' onclick='update_dan("+jsd_display[i].단어번호+","+i+");'>*</a>";
			var temp_str_2 = "<a href='#' onclick='view_dan("+jsd_display[i].단어번호+","+i+");'>*</a>";
			$$("mdan_content").addView
			(				
				{template:"<b>"+temp_str_1+jsd_display[i].제목+temp_str_2+"</b>", autoheight:true, borderless:true,}				
			);
	
			if (!(jsd_display[i].핵심어==null || jsd_display[i].핵심어==""))
			{
				$$("mdan_content").addView
				(
					{template:"<pre><font color='900c3f'>"+jsd_display[i].핵심어+"</font></pre>", autoheight:true, borderless:true}
				);
			}
			
			if ( ($$('config_combobox_display').getValue()=="JHN" || $$('config_combobox_display').getValue()=="JHNH"))	
			if (!(jsd_display[i].내용==null || jsd_display[i].내용==""))
				$$("mdan_content").addView
				(
					{template:jsd_display[i].내용, autoheight:true, borderless:true}
				);
	
			if ($$('config_combobox_display').getValue()=="JHNH")
			if (!(jsd_display[i].히스토리==null || jsd_display[i].히스토리==""))	
				$$("mdan_content").addView
				(
					{template:"<pre>"+jsd_display[i].히스토리+"</pre>", autoheight:true, borderless:true}
				);

			$$("mdan_content").addView({height:5});
		}
	}
	
	var tot_str = cur_page+"/"+parseInt((jsd_display.length+page_num-1)/page_num)+", "+jsd_display.length+"개, 분할:"+$$('config_text_div_num').getValue()+", 페이지당:"+$$('config_text_page_num').getValue();

	$$("mdan_content").addView(
		{
			cols:
			[
				{},
				{template:tot_str, autoheight:true, borderless:true, width:250}
			]
		}
	);				

	$$("mdan_content").addView({});
}

function go_pre()
{
	if (jsd_display.length < 1)
	{
		webix.message("데이타가 없습니다.");
		return 1;
	}

	if (cur_page <= 1)
	{
		webix.message("처음 페이지 입니다.");
		return 1;
	}

	cur_page = cur_page - 1;
	display_dan();
}

function go_next()
{
	var page_num = parseInt($$('config_text_page_num').getValue());

	if (jsd_display.length < 1)
	{
		webix.message("데이타가 없습니다.");
		return 1;
	}

	if (cur_page >= parseInt((jsd_display.length+page_num-1)/page_num) )
	{
		webix.message("마지막 페이지 입니다.");
		return 1;
	}

	cur_page = cur_page + 1;
	display_dan();
}

function go_first()
{
	cur_page = 1;
	display_dan();
}

function go_last()
{
	var page_num = parseInt($$('config_text_page_num').getValue());

	cur_page = parseInt((jsd_display.length+page_num-1)/page_num);
	display_dan();
}


function read_mdan_imsi()
{
	var send_json = { 
		"para1":
		{
			"단어번호":$$('input_text_dan_num').getValue(), 
		}
	};

	webix.ajax().headers({"Content-type":"application/json"}).post("/mdan_10_ar/read_mdan_imsi", send_json, function(text)
	{		
		var jsd = JSON.parse(text)[0];

		$$("input_combobox_group").setValue(jsd.분류);
		$$("input_text_title").setValue(jsd.제목);
		$$("input_text_point").setValue(jsd.핵심어);
		$$("input_text_content").setValue(jsd.내용);
		$$("input_text_history").setValue(jsd.히스토리);

		webix.message(jsd.단어번호);
	});	
}