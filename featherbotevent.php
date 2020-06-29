<?php
error_reporting(E_ALL);
ini_set('display_errors', 1);
$rec = file_get_contents('php://input');
$data = json_decode($rec, true);
//shell_exec('sudo telegrambotreport '.urlencode($rec));
if (empty($data['message']['chat']['id'])) {
	exit();
}
define('TOKEN', '1310579045:AAFXY0-67PYYX6YCtTdxa_-LJZ8cBIJ4oF0');

// Функция вызова методов API.
function sendTelegram($method, $response)
{
	$ch = curl_init('https://api.telegram.org/bot' . TOKEN . '/' . $method);
	curl_setopt($ch, CURLOPT_POST, 1);
	curl_setopt($ch, CURLOPT_POSTFIELDS, $response);
	curl_setopt($ch, CURLOPT_RETURNTRANSFER, true);
	curl_setopt($ch, CURLOPT_HEADER, false);
	$res = curl_exec($ch);
	curl_close($ch);
	return $res;
}

// Прислали файл.
if (!empty($data['message']['document'])) {
	$res = sendTelegram(
		'getFile',
		array(
			'file_id' => $data['message']['document']['file_id']
		)
	);
	
	$res = json_decode($res, true);
	if ($res['ok']) {
		$src = 'https://api.telegram.org/file/bot' . TOKEN . '/' . $res['result']['file_path'];
		$dest = '/var/www/featherbotdata/' . time() . '-' . $data['message']['document']['file_name'];
		if (copy($src, $dest)) {
			sendTelegram(
				'sendMessage', 
				array(
					'chat_id' => $data['message']['chat']['id'],
					'text' => '🖼Анализ файла...'
				)
			);
			$name = 'Feather_Output';
			if (!empty($data['message']['caption'])){
				$name = str_replace(' ', '_', $data['message']['caption']);
			}

			$cmd = 'sudo feather -i -b ' . $dest . ' ' . $name;
			exec($cmd . " 2>&1", $result);
			var_dump($result);
			$addr = implode(',', $result);
			//$addr = shell_exec('sudo feather -i -b ' . $dest . ' ' . $name);
			//$addr = '/var/www/html/api/feather_out_115428.png';
			if (strpos($addr, 'panic') !== false) {
				sendTelegram(
                                	'sendMessage',
                                	array(
                                        	'chat_id' => $data['message']['chat']['id'],
                                        	'text' => ('❌Произошла ошибка: ' . $addr)
                                	)
                        	);
			}
			else {
				sendTelegram(
                	        	'sendPhoto',
                        		array(
                                		'chat_id' => $data['message']['chat']['id'],
                                		'photo' => curl_file_create(trim(preg_replace('/\s\s+/', '', $addr)))
                        		)
                		);
			}
		}
	}
exit();
}

// Ответ на текстовые сообщения.
if (!empty($data['message']['text'])) {
	$text = $data['message']['text'];

	if ($text == '/start') {
		sendTelegram(
			'sendMessage',
			array(
				'chat_id' => $data['message']['chat']['id'],
				'text' => 'Привет! Отправь мне CP1251-закодированный .txt файл электронной книги, а я посчитаю упоминания цветов в ней. Книги такого формата можно найти, например, на https://litres.ru. В описании вложения следует указать название книги.'
			)
		);
		exit();
	}
}
