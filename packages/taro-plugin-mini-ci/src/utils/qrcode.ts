/* eslint-disable no-console */
import { existsSync } from 'node:fs'

import axios from 'axios'
import * as Jimp from 'jimp'
import jsQR from 'jsqr'
import * as QRCode from 'qrcode'

/**
 * 读取出二维码图片中的文本内容
 * @param imagePath 图片路径
 */
export async function readQrcodeImageContent (imagePath: string): Promise<string> {
  // 网络图片优先用axios获取，避免某些云环境下由于原生fetch对代理支持度不够引起的问题
  let imageBuffer: Buffer
  if (!existsSync(imagePath)) {
    try {
      const response = await axios({
        method: 'get',
        url: imagePath,
        responseType: 'arraybuffer',
        timeout: 8000
      })
      imageBuffer = response.data
    } catch (error) {
      console.log(`axios获取图片失败: ${error.message}`)
    }
  }

  return new Promise((resolve, reject) => {
    Jimp.read(imageBuffer || imagePath, function (err, image) {
      if (err) {
        reject(err)
        return
      }
      // @ts-ignore
      const scanData = jsQR(image.bitmap.data, image.bitmap.width, image.bitmap.height)

      if (scanData) {
        resolve(scanData.data)
      } else {
        reject(new Error('扫描器 jsqr 未能识别出二维码内容'))
      }
    })
  })
}

/**
 * 将文本内容转换成二维码输出在控制台上
 * @param content
 */
export async function printQrcode2Terminal (content: string) {
  const terminalStr = await QRCode.toString(content, { type: 'terminal', small: true })
  // eslint-disable-next-line no-console
  console.log(terminalStr)
}

/**
 * 生成二维码图片到指定目录
 * @param content
 * @param path
 */
export async function generateQrcodeImageFile (path: string, content: string) {
  await QRCode.toFile(path, content, {
    errorCorrectionLevel: 'L',
    type: 'png'
  })
}
